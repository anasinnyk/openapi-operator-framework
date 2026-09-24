use crate::ir::ProviderIr;

mod errors;
mod naming;
mod resource;
mod schema;

pub fn generate(ir: &ProviderIr) -> Result<Vec<resource::GeneratedFile>, errors::GeneratorError> {
    let mut files = ir
        .resources
        .iter()
        .map(|(name, res)| resource::generate_resource(ir, name, res))
        .collect::<Result<Vec<_>, _>>()?;

    files.push(resource::generate_client_file(ir)?);
    files.push(resource::generate_mod_file(ir)?);

    Ok(files)
}
