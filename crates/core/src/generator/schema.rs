use crate::{
    generator::{
        errors::GeneratorError,
        naming::{field_ident, type_ident},
    },
    ir::{AdditionalPropertiesIr, FieldIr, OperationIr, ProviderIr, SchemaIr, SchemaName},
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    String,
    Integer,
    Number,
    Boolean,
}

enum ScalarMatch {
    Any,
    Scalar(ScalarKind),
    NonScalar,
}

pub struct OperationTypes {
    pub request: Option<TokenStream>,
    pub response: Option<TokenStream>,
}

pub struct SchemaCodegen<'a> {
    schemas: &'a BTreeMap<SchemaName, SchemaIr>,
    declarations: Vec<TokenStream>,
    generated: BTreeSet<String>,
    generating: BTreeSet<String>,
}

impl<'a> SchemaCodegen<'a> {
    pub fn new(schemas: &'a BTreeMap<SchemaName, SchemaIr>) -> Self {
        Self {
            schemas,
            declarations: Vec::new(),
            generated: BTreeSet::new(),
            generating: BTreeSet::new(),
        }
    }
}

impl SchemaCodegen<'_> {
    pub fn rust_type(
        &mut self,
        schema: &SchemaIr,
        name_hint: &str,
    ) -> Result<TokenStream, GeneratorError> {
        match schema {
            SchemaIr::Ref { target } => {
                let ident = type_ident(&target.0)?;
                let key = ident.to_string();

                if self.generating.contains(&key) {
                    return Ok(quote!(Box<#ident>));
                }

                let ident = self.ensure_schema_generated(target)?;
                Ok(quote!(#ident))
            }

            SchemaIr::Array { items } => {
                let item_type = self.rust_type(items, &format!("{name_hint}Item"))?;

                Ok(quote!(Vec<#item_type>))
            }

            SchemaIr::String { .. } => Ok(quote!(String)),

            SchemaIr::Integer { format } => match format.as_deref() {
                Some("int32") => Ok(quote!(i32)),
                Some("uint32") => Ok(quote!(u32)),
                Some("uint64") => Ok(quote!(u64)),
                _ => Ok(quote!(i64)),
            },

            SchemaIr::Number { format } => match format.as_deref() {
                Some("float") => Ok(quote!(f32)),
                _ => Ok(quote!(f64)),
            },

            SchemaIr::Boolean => Ok(quote!(bool)),

            SchemaIr::Any => Ok(quote!(serde_json::Value)),

            SchemaIr::Object {
                properties,
                additional_properties,
            } => self.generate_inline_object(name_hint, properties, additional_properties),

            SchemaIr::OneOf { variants } => self.generate_one_of(name_hint, variants),

            SchemaIr::AllOf { variants } => self.generate_all_of(name_hint, variants),
        }
    }

    fn one_of_choices(
        &self,
        schema: &SchemaIr,
        visited: &mut BTreeSet<SchemaName>,
    ) -> Option<Vec<SchemaIr>> {
        match schema {
            SchemaIr::OneOf { variants } => Some(variants.clone()),

            SchemaIr::Ref { target } => {
                if !visited.insert(target.clone()) {
                    return None;
                }

                let result = self
                    .schemas
                    .get(target)
                    .and_then(|schema| self.one_of_choices(schema, visited));

                visited.remove(target);
                result
            }

            _ => None,
        }
    }

    fn distribute_all_of(&self, variants: &[SchemaIr]) -> Option<SchemaIr> {
        let mut found_one_of = false;
        let mut combinations = vec![Vec::new()];

        for variant in variants {
            let choices = self.one_of_choices(variant, &mut BTreeSet::new());

            if let Some(choices) = choices {
                found_one_of = true;

                combinations = combinations
                    .into_iter()
                    .flat_map(|combination| {
                        choices.iter().map(move |choice| {
                            let mut result = combination.clone();
                            result.push(choice.clone());
                            result
                        })
                    })
                    .collect();
            } else {
                for combination in &mut combinations {
                    combination.push(variant.clone());
                }
            }
        }

        found_one_of.then(|| SchemaIr::OneOf {
            variants: combinations
                .into_iter()
                .map(|variants| SchemaIr::AllOf { variants })
                .collect(),
        })
    }

    pub fn generate_operation_types(
        &mut self,
        operation_id: &str,
        operation: &OperationIr,
    ) -> Result<OperationTypes, GeneratorError> {
        let operation_name = operation_id.to_upper_camel_case();

        let request = operation
            .request_body
            .as_ref()
            .map(|schema| self.rust_type(schema, &format!("{operation_name}Request")))
            .transpose()?;

        let response = operation
            .response_body
            .as_ref()
            .map(|schema| self.rust_type(schema, &format!("{operation_name}Response")))
            .transpose()?;

        Ok(OperationTypes { request, response })
    }

    fn generate_one_of(
        &mut self,
        name_hint: &str,
        variants: &[SchemaIr],
    ) -> Result<TokenStream, GeneratorError> {
        if variants.is_empty() {
            return Err(GeneratorError::UnsupportedSchema(
                "oneOf must contain at least one variant".into(),
            ));
        }

        let enum_ident = type_ident(name_hint)?;
        let key = enum_ident.to_string();

        if self.generated.insert(key) {
            let variants = variants
                .iter()
                .enumerate()
                .map(|(index, schema)| {
                    let number = index + 1;

                    let variant_ident = type_ident(&format!("Variant{number}"))?;

                    let variant_type =
                        self.rust_type(schema, &format!("{name_hint}Variant{number}"))?;

                    Ok(quote! {
                        #variant_ident(#variant_type)
                    })
                })
                .collect::<Result<Vec<_>, GeneratorError>>()?;

            self.declarations.push(quote! {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    serde::Serialize,
                    serde::Deserialize,
                    schemars::JsonSchema
                )]
                #[serde(untagged)]
                pub enum #enum_ident {
                    #(#variants),*
                }
            });
        }

        Ok(quote!(#enum_ident))
    }

    pub fn object_properties(
        &self,
        schema: &SchemaIr,
    ) -> Result<BTreeMap<String, FieldIr>, GeneratorError> {
        let mut properties = BTreeMap::new();
        let mut visiting = BTreeSet::new();

        self.collect_object_properties(schema, &mut properties, &mut visiting)?;

        Ok(properties)
    }

    fn scalar_match(&self, schema: &SchemaIr) -> Result<ScalarMatch, GeneratorError> {
        match schema {
            SchemaIr::String { .. } => Ok(ScalarMatch::Scalar(ScalarKind::String)),

            SchemaIr::Integer { .. } => Ok(ScalarMatch::Scalar(ScalarKind::Integer)),

            SchemaIr::Number { .. } => Ok(ScalarMatch::Scalar(ScalarKind::Number)),

            SchemaIr::Boolean => Ok(ScalarMatch::Scalar(ScalarKind::Boolean)),

            SchemaIr::Any => Ok(ScalarMatch::Any),

            SchemaIr::Ref { target } => {
                let target = self
                    .schemas
                    .get(target)
                    .ok_or_else(|| GeneratorError::UnknownSchema(target.0.clone()))?;

                self.scalar_match(target)
            }

            SchemaIr::AllOf { variants } => {
                let mut result = ScalarMatch::Any;

                for variant in variants {
                    match self.scalar_match(variant)? {
                        ScalarMatch::Any => {}

                        ScalarMatch::Scalar(kind) => match result {
                            ScalarMatch::Any => {
                                result = ScalarMatch::Scalar(kind);
                            }

                            ScalarMatch::Scalar(previous) if previous == kind => {}

                            _ => return Ok(ScalarMatch::NonScalar),
                        },

                        ScalarMatch::NonScalar => {
                            return Ok(ScalarMatch::NonScalar);
                        }
                    }
                }

                Ok(result)
            }

            _ => Ok(ScalarMatch::NonScalar),
        }
    }

    fn ensure_schema_generated(
        &mut self,
        target: &SchemaName,
    ) -> Result<proc_macro2::Ident, GeneratorError> {
        let ident = type_ident(&target.0)?;
        let key = ident.to_string();

        if self.generated.contains(&key) {
            return Ok(ident);
        }

        if !self.generating.insert(key.clone()) {
            return Err(GeneratorError::CyclicSchema(target.0.clone()));
        }

        // Clone потрібен, бо нижче ми mutable borrow self.
        let schema = self
            .schemas
            .get(target)
            .cloned()
            .ok_or_else(|| GeneratorError::UnknownSchema(target.0.clone()))?;

        let result = self.generate_named_schema(&ident, &target.0, &schema);

        self.generating.remove(&key);

        result?;

        Ok(ident)
    }

    fn generate_named_schema(
        &mut self,
        ident: &proc_macro2::Ident,
        name: &str,
        schema: &SchemaIr,
    ) -> Result<(), GeneratorError> {
        let key = ident.to_string();

        let rust_type = match schema {
            SchemaIr::Object {
                properties,
                additional_properties,
            } => self.generate_inline_object(name, properties, additional_properties)?,

            SchemaIr::OneOf { variants } => self.generate_one_of(name, variants)?,

            SchemaIr::AllOf { variants } => self.generate_all_of(name, variants)?,

            schema => self.rust_type(schema, name)?,
        };

        if !self.generated.contains(&key) {
            self.declarations.push(quote! {
                pub type #ident = #rust_type;
            });

            self.generated.insert(key);
        }

        Ok(())
    }

    fn generate_all_of(
        &mut self,
        name_hint: &str,
        variants: &[SchemaIr],
    ) -> Result<TokenStream, GeneratorError> {
        if let Some(distributed) = self.distribute_all_of(variants) {
            return self.rust_type(&distributed, name_hint);
        }
        let mut scalar_kind = None;
        let mut scalar_only = true;

        for variant in variants {
            match self.scalar_match(variant)? {
                ScalarMatch::Any => {}

                ScalarMatch::Scalar(kind) => {
                    if let Some(previous) = scalar_kind {
                        if previous != kind {
                            return Err(GeneratorError::UnsupportedSchema(format!(
                                "allOf mixes incompatible scalar types: \
                                     {previous:?} and {kind:?}"
                            )));
                        }
                    } else {
                        scalar_kind = Some(kind);
                    }
                }

                ScalarMatch::NonScalar => {
                    scalar_only = false;
                    break;
                }
            }
        }

        if scalar_only {
            return Ok(match scalar_kind {
                Some(ScalarKind::String) => quote!(String),
                Some(ScalarKind::Integer) => quote!(i64),
                Some(ScalarKind::Number) => quote!(f64),
                Some(ScalarKind::Boolean) => quote!(bool),
                None => quote!(serde_json::Value),
            });
        }

        let schema = SchemaIr::AllOf {
            variants: variants.to_vec(),
        };

        let properties = self.object_properties(&schema)?;

        self.generate_inline_object(name_hint, &properties, &AdditionalPropertiesIr::Forbidden)
    }

    fn collect_object_properties(
        &self,
        schema: &SchemaIr,
        output: &mut BTreeMap<String, FieldIr>,
        visiting: &mut BTreeSet<SchemaName>,
    ) -> Result<(), GeneratorError> {
        match schema {
            SchemaIr::Object { properties, .. } => {
                for (name, field) in properties {
                    merge_field(output, name, field)?;
                }

                Ok(())
            }

            SchemaIr::AllOf { variants } => {
                for variant in variants {
                    self.collect_object_properties(variant, output, visiting)?;
                }

                Ok(())
            }

            SchemaIr::Any => Ok(()),

            SchemaIr::Ref { target } => {
                if !visiting.insert(target.clone()) {
                    return Err(GeneratorError::UnsupportedSchema(format!(
                        "cyclic schema reference: {}",
                        target.0
                    )));
                }

                let schema = self
                    .schemas
                    .get(target)
                    .ok_or_else(|| GeneratorError::UnknownSchema(target.0.clone()))?;

                self.collect_object_properties(schema, output, visiting)?;

                visiting.remove(target);

                Ok(())
            }

            other => Err(GeneratorError::UnsupportedSchema(format!(
                "allOf variant must be an object or object reference, got: {other:#?}"
            ))),
        }
    }

    pub fn generate_fields(
        &mut self,
        parent_name: &str,
        properties: &BTreeMap<String, FieldIr>,
    ) -> Result<Vec<TokenStream>, GeneratorError> {
        properties
            .iter()
            .map(|(json_name, field)| {
                let ident = field_ident(json_name)?;

                let name_hint = format!("{parent_name}{}", type_ident(json_name)?);

                let mut field_type = self.rust_type(&field.schema, &name_hint)?;

                let optional = !field.required || field.nullable;

                if optional {
                    field_type = quote!(Option<#field_type>);
                }

                let serde = if optional {
                    quote! {
                        #[serde(
                            rename = #json_name,
                            default,
                            skip_serializing_if = "Option::is_none"
                        )]
                    }
                } else {
                    quote! {
                        #[serde(rename = #json_name)]
                    }
                };

                let documentation = field
                    .description
                    .as_ref()
                    .map(|description| quote!(#[doc = #description]));

                Ok(quote! {
                    #documentation
                    #serde
                    pub #ident: #field_type,
                })
            })
            .collect()
    }

    pub fn finish(self) -> Vec<TokenStream> {
        self.declarations
    }

    fn generate_inline_object(
        &mut self,
        name_hint: &str,
        properties: &BTreeMap<String, FieldIr>,
        additional: &AdditionalPropertiesIr,
    ) -> Result<TokenStream, GeneratorError> {
        let ident = type_ident(name_hint)?;
        let key = ident.to_string();

        if self.generated.insert(key) {
            let mut fields = self.generate_fields(name_hint, properties)?;

            if let Some(field) = self.additional_properties_field(name_hint, additional)? {
                fields.push(field);
            }

            self.declarations.push(quote! {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    serde::Serialize,
                    serde::Deserialize,
                    schemars::JsonSchema
                )]
                pub struct #ident {
                    #(#fields)*
                }
            });
        }

        Ok(quote!(#ident))
    }

    fn additional_properties_field(
        &mut self,
        name_hint: &str,
        additional: &AdditionalPropertiesIr,
    ) -> Result<Option<TokenStream>, GeneratorError> {
        let value_type = match additional {
            AdditionalPropertiesIr::Forbidden => return Ok(None),

            AdditionalPropertiesIr::Any => {
                quote!(serde_json::Value)
            }

            AdditionalPropertiesIr::Typed { schema } => {
                self.rust_type(schema, &format!("{name_hint}AdditionalProperty"))?
            }
        };

        Ok(Some(quote! {
            #[serde(
                flatten,
                default,
                skip_serializing_if =
                    "std::collections::BTreeMap::is_empty"
            )]
            pub additional_properties:
                std::collections::BTreeMap<String, #value_type>,
        }))
    }
}

fn is_unconstrained_object(schema: &SchemaIr) -> bool {
    matches!(
        schema,
        SchemaIr::Object {
            properties,
            additional_properties: AdditionalPropertiesIr::Any,
        } if properties.is_empty()
    )
}

fn merge_field(
    output: &mut BTreeMap<String, FieldIr>,
    name: &str,
    field: &FieldIr,
) -> Result<(), GeneratorError> {
    let Some(existing) = output.get_mut(name) else {
        output.insert(name.to_owned(), field.clone());
        return Ok(());
    };

    if matches!(&existing.schema, SchemaIr::Any) || is_unconstrained_object(&existing.schema) {
        existing.schema = field.schema.clone();
    } else if matches!(&field.schema, SchemaIr::Any) || is_unconstrained_object(&field.schema) {
    } else if existing.schema != field.schema {
        return Err(GeneratorError::UnsupportedSchema(format!(
            "allOf contains incompatible definitions of field {name}: \
         existing={:#?}, incoming={:#?}",
            existing.schema, field.schema,
        )));
    }

    existing.required |= field.required;
    existing.nullable &= field.nullable;

    if existing.description.is_none() {
        existing.description.clone_from(&field.description);
    }

    if existing.default.is_none() {
        existing.default.clone_from(&field.default);
    }

    Ok(())
}

pub fn generate_http_client(
    ir: &ProviderIr,
    schema_codegen: &mut SchemaCodegen<'_>,
) -> Result<TokenStream, GeneratorError> {
    let mut methods = Vec::new();

    for (operation_id, operation) in &ir.operations {
        let operation_id = operation_id.as_str();
        let method_ident = field_ident(&operation_id.to_snake_case())?;

        let operation_types = schema_codegen.generate_operation_types(operation_id, operation)?;

        let response_type = operation_types.response.unwrap_or_else(|| quote!(()));

        let path_parameters = operation
            .path_parameters
            .keys()
            .map(|name| {
                let ident = field_ident(name)?;
                Ok((name, ident))
            })
            .collect::<Result<Vec<_>, GeneratorError>>()?;

        let arguments = path_parameters
            .iter()
            .map(|(_, ident)| quote!(#ident: &str));

        let replacements = path_parameters.iter().map(|(name, ident)| {
            let placeholder = format!("{{{name}}}");

            quote! {
                path = path.replace(
                    #placeholder,
                    urlencoding::encode(#ident).as_ref(),
                );
            }
        });

        let (body_argument, apply_body) = match operation_types.request {
            Some(request_type) => (
                Some(quote!(body: &#request_type)),
                quote!(request = request.json(body);),
            ),
            None => (None, quote!()),
        };

        let path = &operation.path;
        let method = operation.method.as_str();

        methods.push(quote! {
            pub fn #method_ident(
                &self,
                #(#arguments,)*
                #body_argument
            ) -> core::ApiRequest<#response_type> {
                let mut path = #path.to_owned();
                #(#replacements)*

                let url = format!(
                    "{}{}",
                    self.base_url.trim_end_matches('/'),
                    path,
                );

                let method = reqwest::Method::from_bytes(
                    #method.as_bytes()
                ).expect("validated HTTP method");

                let mut request = self.http.request(method, url);
                #apply_body

                core::ApiRequest::new(request)
            }
        });
    }

    Ok(quote! {
        #[derive(Clone)]
        pub struct ProviderClient {
            http: reqwest::Client,
            base_url: String,
        }

        impl ProviderClient {
            pub fn new(
                http: reqwest::Client,
                base_url: impl Into<String>,
            ) -> Self {
                Self {
                    http,
                    base_url: base_url.into(),
                }
            }

            #(#methods)*
        }
    })
}
