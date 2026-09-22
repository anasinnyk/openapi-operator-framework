use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, type_ident},
        schema::SchemaCodegen,
    },
    ir::{ProviderIr, ResourceIr, ResourceName, ValueExpr},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
}

pub fn generate_resource(
    ir: &ProviderIr,
    name: &ResourceName,
    resource: &ResourceIr,
) -> Result<GeneratedFile, GeneratorError> {
    let spec = generate_spec(ir, resource)?;
    let status = generate_status(resource)?;
    let custom_resource = generate_custom_resource(&spec, &status);

    let content = format_tokens(custom_resource)?;

    Ok(GeneratedFile {
        path: format!("src/generated/{}.rs", name.0).into(),
        content,
    })
}

fn format_tokens(tokens: TokenStream) -> Result<String, GeneratorError> {
    let syntax: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax))
}

pub fn generate_spec(
    ir: &ProviderIr,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let schema = ir
        .schemas
        .get(&resource.spec_schema)
        .ok_or_else(|| GeneratorError::UnknownSchema(resource.spec_schema.0.clone()))?;

    let (group, version) = resource
        .api_version
        .split_once('/')
        .ok_or_else(|| GeneratorError::InvalidApiVersion(resource.api_version.clone()))?;

    let kind = &resource.kind;
    let status_name = format!("{}Status", resource.kind);

    let mut codegen = SchemaCodegen::new(&ir.schemas);

    let properties = codegen.object_properties(schema)?;
    let for_provider_name = format!("{}ForProvider", resource.kind);

    let for_provider_ident = type_ident(&for_provider_name)?;

    let spec_ident = type_ident(&format!("{}Spec", resource.kind))?;

    let managed_properties = properties
        .iter()
        .filter(|(_, field)| !field.read_only && !field.ignored)
        .map(|(name, field)| (name.clone(), field.clone()))
        .collect::<BTreeMap<_, _>>();

    let fields = codegen.generate_fields(&for_provider_name, &managed_properties)?;

    let declarations = codegen.finish();

    Ok(quote! {
        #(#declarations)*

        #[derive(
            Clone,
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        pub struct #for_provider_ident {
            #(#fields)*
        }

        #[derive(
            kube::CustomResource,
            Clone,
            Debug,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        #[kube(
            group = #group,
            version = #version,
            kind = #kind,
            namespaced,
            status = #status_name
        )]
        pub struct #spec_ident {
            #[serde(rename = "forProvider")]
            pub for_provider: #for_provider_ident,

            #[serde(flatten)]
            pub management:
                core::managed::ManagedResourceSpec,
        }
    })
}

pub fn generate_status(resource: &ResourceIr) -> Result<TokenStream, GeneratorError> {
    let status_ident = type_ident(&format!("{}Status", resource.kind))?;
    let at_provider_ident = type_ident(&format!("{}AtProvider", resource.kind))?;

    let mut identifier_fields = BTreeSet::new();

    for expression in resource.identifiers.values() {
        let ValueExpr::Field { path } = expression else {
            return Err(GeneratorError::UnsupportedSchema(
                "resource identifier must be a field expression".into(),
            ));
        };

        let name = path.0.strip_prefix("status.atProvider.").ok_or_else(|| {
            GeneratorError::UnsupportedSchema(format!(
                "identifier path must start with status.atProvider.: {}",
                path.0
            ))
        })?;

        if name.contains('.') {
            return Err(GeneratorError::UnsupportedSchema(format!(
                "nested status path is not supported yet: {}",
                path.0
            )));
        }

        identifier_fields.insert(name.to_owned());
    }

    let identifier_fields = identifier_fields
        .into_iter()
        .map(|json_name| {
            let ident = field_ident(&json_name)?;

            Ok(quote! {
                #[serde(
                    rename = #json_name,
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub #ident: Option<String>,
            })
        })
        .collect::<Result<Vec<_>, GeneratorError>>()?;

    Ok(quote! {
        #[derive(
            Clone,
            Debug,
            Default,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        pub struct #at_provider_ident {
            #(#identifier_fields)*
        }

        #[derive(
            Clone,
            Debug,
            Default,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema
        )]
        pub struct #status_ident {
            #[serde(
                rename = "atProvider",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub at_provider: Option<#at_provider_ident>,

            #[serde(
                rename = "observedGeneration",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub observed_generation: Option<i64>,

            #[serde(default)]
            pub conditions: Vec<
                k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition
            >,
        }
    })
}

fn generate_custom_resource(spec: &TokenStream, status: &TokenStream) -> TokenStream {
    quote! {
        // This file is generated. Do not edit manually.

        #status
        #spec
    }
}
