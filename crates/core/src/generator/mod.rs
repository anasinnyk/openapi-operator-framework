use crate::ir::ProviderIr;

mod errors;
mod naming;
mod resource;
mod schema;

pub fn generate(ir: &ProviderIr) -> Result<Vec<resource::GeneratedFile>, errors::GeneratorError> {
    ir.resources
        .iter()
        .map(|(name, res)| resource::generate_resource(ir, name, res))
        .collect()
}
