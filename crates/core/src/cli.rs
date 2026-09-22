use crate::{generator::generate, loader::load, validation::validate};
use anyhow::{Context, Result, bail};
use clap::Parser;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(name = "provider-generator")]
#[command(about = "Generate Kubernetes provider code from OpenAPI")]
pub struct Args {
    /// Source OpenAPI YAML or JSON file.
    #[arg(short = 'i', long, value_name = "FILE")]
    openapi: PathBuf,

    /// Overlay files applied in the specified order.
    #[arg(long, value_name = "FILE")]
    overlay: Vec<PathBuf>,

    /// Directory for generated files.
    #[arg(short = 'o', long, value_name = "DIRECTORY")]
    output: PathBuf,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let ir = load(&args.openapi, &args.overlay)?;

    if let Err(errors) = validate(&ir) {
        bail!("invalid ProviderIr:\n{errors:#?}");
    }

    let files = generate(&ir)?;

    for file in files {
        write_generated_file(&args.output, &file.path, &file.content)?;
    }

    Ok(())
}

fn write_generated_file(output: &Path, relative_path: &Path, content: &str) -> Result<()> {
    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        bail!(
            "generated path must be relative: {}",
            relative_path.display()
        );
    }

    let path = output.join(relative_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("cannot create {}", parent.display()))?;
    }

    fs::write(&path, content).with_context(|| format!("cannot write {}", path.display()))?;

    Ok(())
}
