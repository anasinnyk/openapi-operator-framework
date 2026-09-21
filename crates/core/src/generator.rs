use crate::ir::ProviderIr;
use std::fmt::Write;
use std::path::PathBuf;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

pub fn generate(ir: &ProviderIr) -> Vec<GeneratedFile> {
    let mut output = String::new();
    let mut files = Vec::with_capacity(ir.resources.len());

    for (name, resource) in &ir.resources {
        let _ = writeln!(
            output,
            "pub const {}_KIND: &str = {:?};",
            name.0.to_uppercase(),
            resource.kind,
        );

        files.push(GeneratedFile {
            path: format!("src/generated/{}.rs", resource.kind.to_lowercase()).into(),
            content: output.clone(),
        });
    }

    files
}
