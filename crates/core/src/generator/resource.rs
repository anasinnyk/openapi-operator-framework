use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, type_ident},
        schema::{SchemaCodegen, generate_http_client},
    },
    ir::{
        ApiKeyLocationIr, CredentialSourceIr, FieldPath, ProviderIr, ResourceIr, ResourceName,
        SecuritySchemeIr, ValueExpr,
    },
};
use heck::ToUpperCamelCase;
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
    let credentials = generate_resolve_credentials(ir, name, resource)?;

    let custom_resource = generate_custom_resource(&spec, &status, &credentials);

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

fn generate_spec(ir: &ProviderIr, resource: &ResourceIr) -> Result<TokenStream, GeneratorError> {
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

    let mut fields = codegen.generate_fields(&for_provider_name, &managed_properties)?;
    fields.extend(generate_reference_fields(resource)?);

    if let Some(credentials) = generate_credential_field(resource)? {
        fields.push(credentials);
    }

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

fn generate_status(resource: &ResourceIr) -> Result<TokenStream, GeneratorError> {
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

fn generate_custom_resource(
    spec: &TokenStream,
    status: &TokenStream,
    credentials: &TokenStream,
) -> TokenStream {
    quote! {
        // This file is generated. Do not edit manually.

        #status
        #spec

        #credentials
    }
}

fn generate_reference_fields(resource: &ResourceIr) -> Result<Vec<TokenStream>, GeneratorError> {
    resource
        .references
        .values()
        .map(|reference| {
            let json_name = for_provider_field(&reference.from)?;
            let ident = field_ident(json_name)?;

            Ok(quote! {
                #[serde(rename = #json_name)]
                pub #ident:
                    core::reference::ResourceReference,
            })
        })
        .collect()
}

fn generate_credential_field(resource: &ResourceIr) -> Result<Option<TokenStream>, GeneratorError> {
    let Some(credentials) = &resource.credentials else {
        return Ok(None);
    };

    let CredentialSourceIr::SecretKeySelector { path } = &credentials.source else {
        return Ok(None);
    };

    let json_name = for_provider_field(path)?;
    let ident = field_ident(json_name)?;

    Ok(Some(quote! {
        #[serde(rename = #json_name)]
        pub #ident:
            core::reference::SecretReference,
    }))
}

fn for_provider_field(path: &FieldPath) -> Result<&str, GeneratorError> {
    let field = path.0.strip_prefix("spec.forProvider.").ok_or_else(|| {
        GeneratorError::UnsupportedSchema(format!(
            "reference path must start with \
                 spec.forProvider.: {}",
            path.0
        ))
    })?;

    if field.contains('.') {
        return Err(GeneratorError::UnsupportedSchema(format!(
            "nested reference paths are not supported: {}",
            path.0
        )));
    }

    Ok(field)
}

fn field_path_access(root: TokenStream, path: &FieldPath) -> Result<TokenStream, GeneratorError> {
    path.0.split('.').try_fold(root, |access, segment| {
        let field = field_ident(segment)?;
        Ok(quote!(#access.#field))
    })
}

pub fn generate_resolve_credentials(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    resource: &ResourceIr,
) -> Result<TokenStream, GeneratorError> {
    let Some(credentials) = &resource.credentials else {
        return Ok(TokenStream::new());
    };

    let resource_ident = type_ident(&resource.kind)?;

    let credential_ident = type_ident(&format!(
        "{}Credential",
        credentials.security_scheme.0.to_upper_camel_case(),
    ))?;

    let mut traversal = Vec::new();
    let mut current_resource = resource;
    let mut current_value = quote!(resource);
    let mut current_namespace = quote!(namespace);

    match &credentials.source {
        CredentialSourceIr::SecretKeySelector { .. } => {}

        CredentialSourceIr::Related { via } => {
            for (index, relation_name) in via.iter().enumerate() {
                let relation = current_resource
                    .references
                    .get(relation_name)
                    .ok_or_else(|| {
                        GeneratorError::UnsupportedCredential(format!(
                            "resource {} has no relation {}",
                            resource_name.0, relation_name,
                        ))
                    })?;

                let target = ir.resources.get(&relation.target).ok_or_else(|| {
                    GeneratorError::UnsupportedCredential(format!(
                        "unknown related resource {}",
                        relation.target.0,
                    ))
                })?;

                let reference_access = field_path_access(current_value.clone(), &relation.from)?;

                let reference_ident = syn::Ident::new(
                    &format!("reference_{index}"),
                    proc_macro2::Span::call_site(),
                );

                let namespace_ident = syn::Ident::new(
                    &format!("namespace_{index}"),
                    proc_macro2::Span::call_site(),
                );

                let api_ident =
                    syn::Ident::new(&format!("api_{index}"), proc_macro2::Span::call_site());

                let value_ident =
                    syn::Ident::new(&format!("related_{index}"), proc_macro2::Span::call_site());

                let target_module = field_ident(&relation.target.0)?;

                let target_ident = type_ident(&target.kind)?;

                let reference_path = &relation.from.0;

                let target_type = quote! {
                    crate::generated::#target_module::#target_ident
                };

                traversal.push(quote! {
                    let #reference_ident =
                        (#reference_access)
                            .as_ref()
                            .ok_or_else(|| {
                                provider_core::CredentialError::MissingValue(
                                    format!(
                                        "resource reference {} is not set",
                                        #reference_path,
                                    ),
                                )
                            })?;

                    let #namespace_ident = #reference_ident
                        .namespace
                        .clone()
                        .unwrap_or_else(|| #current_namespace.clone());

                    let #api_ident: kube::Api<#target_type> =
                        kube::Api::namespaced(
                            client.clone(),
                            &#namespace_ident,
                        );

                    let #value_ident = #api_ident
                        .get(&#reference_ident.name)
                        .await?;
                });

                current_resource = target;
                current_value = quote!(#value_ident);
                current_namespace = quote!(#namespace_ident);
            }
        }
    }

    let terminal_credentials = current_resource.credentials.as_ref().ok_or_else(|| {
        GeneratorError::UnsupportedCredential(format!(
            "credential relation from {} ends at a resource \
                 without credentials",
            resource_name.0,
        ))
    })?;

    let CredentialSourceIr::SecretKeySelector { path: secret_path } = &terminal_credentials.source
    else {
        return Err(GeneratorError::UnsupportedCredential(format!(
            "credential relation from {} does not end at \
                 direct Secret credentials",
            resource_name.0,
        )));
    };

    let selector_access = field_path_access(current_value, secret_path)?;

    let secret_path_string = &secret_path.0;
    let resource_kind = &resource.kind;

    Ok(quote! {
        pub async fn resolve_credentials(
            client: &kube::Client,
            resource: &#resource_ident,
        ) -> Result<
            crate::generated::client::#credential_ident,
            core::api::CredentialError,
        > {
            let namespace =
                kube::ResourceExt::namespace(resource)
                    .ok_or_else(|| {
                        core::api::CredentialError::MissingValue(
                            format!(
                                "{} has no namespace",
                                #resource_kind,
                            ),
                        )
                    })?;

            #(#traversal)*

            let selector = (#selector_access)
                .as_ref()
                .ok_or_else(|| {
                    core::api::CredentialError::MissingValue(
                        format!(
                            "credential field {} is not set",
                            #secret_path_string,
                        ),
                    )
                })?;

            let value = core::reference::resolve_secret_key(
                client,
                &#current_namespace,
                selector,
            )
            .await?;

            Ok(
                crate::generated::client::#credential_ident::new(
                    value,
                )?
            )
        }
    })
}

pub fn generate_credentials(ir: &ProviderIr) -> Result<TokenStream, GeneratorError> {
    let mut generated = Vec::new();

    for (name, scheme) in &ir.security_schemes {
        let type_name = format!("{}Credential", name.0.to_upper_camel_case());
        let credential_ident = type_ident(&type_name)?;

        let inner_type;
        let constructor;

        match scheme {
            SecuritySchemeIr::Http { scheme, .. } if scheme == "bearer" => {
                inner_type = quote!(provider_core::BearerCredential);

                constructor = quote! {
                    provider_core::BearerCredential::new(value)?
                };
            }

            SecuritySchemeIr::ApiKey {
                name: header_name,
                location: ApiKeyLocationIr::Header,
            } => {
                inner_type = quote!(provider_core::HeaderCredential);

                constructor = quote! {
                    provider_core::HeaderCredential::new(
                        #header_name,
                        value,
                    )?
                };
            }

            _ => {
                return Err(GeneratorError::UnsupportedCredential(format!(
                    "security scheme {} is not supported",
                    name.0,
                )));
            }
        }

        generated.push(quote! {
            pub struct #credential_ident(#inner_type);

            impl #credential_ident {
                pub fn new(
                    value: impl AsRef<str>,
                ) -> Result<
                    Self,
                    provider_core::CredentialError,
                > {
                    Ok(Self(#constructor))
                }
            }

            impl provider_core::ApiCredential
                for #credential_ident
            {
                fn apply(
                    &self,
                    request: reqwest::RequestBuilder,
                ) -> reqwest::RequestBuilder {
                    provider_core::ApiCredential::apply(
                        &self.0,
                        request,
                    )
                }
            }
        });
    }

    Ok(quote! {
        #(#generated)*
    })
}

pub fn generate_client_file(ir: &ProviderIr) -> Result<GeneratedFile, GeneratorError> {
    let mut schema_codegen = SchemaCodegen::new(&ir.schemas);

    let credentials = generate_credentials(ir)?;
    let client = generate_http_client(ir, &mut schema_codegen)?;
    let declarations = schema_codegen.finish();

    let content = quote! {
        #(#declarations)*
        #credentials
        #client
    };

    Ok(GeneratedFile {
        path: "src/generated/client.rs".into(),
        content: format_tokens(content)?,
    })
}
