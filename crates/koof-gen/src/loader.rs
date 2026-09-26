use crate::ir::ProviderIr;
use anyhow::{Context, Result};
use enumset::EnumSet;
use roas_overlay::{
    apply::{Apply, ApplyOptions},
    v1_1::Overlay,
};
use serde_json::Value;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::extractor::extract;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse YAML {path}: {source}")]
    Yaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("failed to apply overlay {path}: {message}")]
    Overlay { path: PathBuf, message: String },

    #[error("failed to build ProviderIr: {0}")]
    Ir(String),

    #[error("invalid OpenAPI: {0}")]
    InvalidOpenApi(String),
}

/// Reads an `OpenAPI` document, applies overlays in the given order and extracts a
/// [`ProviderIr`].
///
/// # Errors
///
/// Returns an error when a file cannot be read or parsed, an overlay action matches
/// nothing, or the resulting document cannot be converted to [`ProviderIr`].
pub fn load(openapi_path: &Path, overlay_paths: &[PathBuf]) -> Result<ProviderIr> {
    let source = fs::read_to_string(openapi_path)
        .with_context(|| format!("cannot read {}", openapi_path.display()))?;

    let mut openapi: Value = serde_yaml::from_str(&source).context("cannot parse OpenAPI")?;

    for overlay_path in overlay_paths {
        let source = fs::read_to_string(overlay_path)
            .with_context(|| format!("cannot read {}", overlay_path.display()))?;

        let overlay: Overlay = serde_yaml::from_str(&source)
            .with_context(|| format!("cannot parse {}", overlay_path.display()))?;

        let options: EnumSet<ApplyOptions> = ApplyOptions::ErrorOnZeroMatch.into();

        overlay
            .apply(&mut openapi, options)
            .map_err(|error| anyhow::anyhow!("cannot apply {}: {error}", overlay_path.display()))?;
    }

    Ok(extract(&openapi)?)
}
