use std::collections::{BTreeMap, BTreeSet};

use crate::ir::{
    AdditionalPropertiesIr, CredentialsIr, FieldIr, HttpMethod, LifecycleIr, OperationId,
    OperationIr, ProviderIr, ReferenceIr, ResourceIr, ResourceName, SchemaIr, SchemaName,
    ValueExpr,
};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::loader::LoaderError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceExtension {
    name: ResourceName,
    api_version: String,
    kind: String,
    #[serde(default)]
    identifiers: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    references: BTreeMap<String, ReferenceIr>,
    credentials: Option<CredentialsIr>,
}

#[derive(Debug, Deserialize)]
struct OperationExtension {
    resource: ResourceName,
    lifecycle: LifecycleAction,
    #[serde(default)]
    path_parameters: BTreeMap<String, ValueExpr>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum LifecycleAction {
    Observe,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Default)]
struct LifecycleDraft {
    observe: Option<OperationId>,
    create: Option<OperationId>,
    update: Option<OperationId>,
    delete: Option<OperationId>,
}

struct ResourceDraft {
    extension: ResourceExtension,
    schema: SchemaName,
    lifecycle: LifecycleDraft,
}

pub fn extract(openapi: &Value) -> Result<ProviderIr, LoaderError> {
    let mut resources = extract_resources(openapi)?;
    let operations = extract_operations(openapi, &mut resources)?;

    let resources = resources
        .into_iter()
        .map(|(name, draft)| {
            let observe = draft
                .lifecycle
                .observe
                .ok_or_else(|| ir_error(format!("resource {} has no observe operation", name.0)))?;

            Ok((
                name,
                ResourceIr {
                    api_version: draft.extension.api_version,
                    kind: draft.extension.kind,
                    spec_schema: draft.schema,
                    identifiers: draft.extension.identifiers,
                    references: draft.extension.references,
                    credentials: draft.extension.credentials,
                    lifecycle: LifecycleIr {
                        observe,
                        create: draft.lifecycle.create,
                        update: draft.lifecycle.update,
                        delete: draft.lifecycle.delete,
                    },
                },
            ))
        })
        .collect::<Result<BTreeMap<_, _>, LoaderError>>()?;

    let provider = openapi
        .pointer("/x-kube-provider/name")
        .or_else(|| openapi.pointer("/info/title"))
        .and_then(Value::as_str)
        .ok_or_else(|| ir_error("missing x-kube-provider.name or info.title"))?
        .to_owned();

    let schemas = extract_schemas(openapi)?;

    Ok(ProviderIr {
        version: 1,
        provider,
        schemas,
        resources,
        operations,
    })
}

fn extract_resources(
    openapi: &Value,
) -> Result<BTreeMap<ResourceName, ResourceDraft>, LoaderError> {
    let schemas = object_at(openapi, "/components/schemas")?;
    let mut resources = BTreeMap::new();

    for (schema_name, schema) in schemas {
        let Some(extension) = schema.get("x-kube-resource") else {
            continue;
        };

        let extension: ResourceExtension =
            serde_json::from_value(extension.clone()).map_err(|error| {
                ir_error(format!(
                    "invalid x-kube-resource on schema {schema_name}: {error}"
                ))
            })?;

        let name = extension.name.clone();
        let draft = ResourceDraft {
            extension,
            schema: SchemaName(schema_name.clone()),
            lifecycle: LifecycleDraft::default(),
        };

        if resources.insert(name.clone(), draft).is_some() {
            return Err(ir_error(format!("duplicate resource {}", name.0)));
        }
    }

    Ok(resources)
}

fn extract_operations(
    openapi: &Value,
    resources: &mut BTreeMap<ResourceName, ResourceDraft>,
) -> Result<BTreeMap<OperationId, OperationIr>, LoaderError> {
    let paths = object_at(openapi, "/paths")?;
    let mut operations = BTreeMap::new();

    for (path, path_item) in paths {
        let Some(path_item) = path_item.as_object() else {
            continue;
        };

        for (method_name, method) in http_operations(path_item) {
            let Some(extension) = method.get("x-kube-operation") else {
                continue;
            };

            let extension: OperationExtension =
                serde_json::from_value(extension.clone()).map_err(|error| {
                    ir_error(format!(
                        "invalid x-kube-operation on {method_name} {path}: {error}"
                    ))
                })?;

            let operation_id = method
                .get("operationId")
                .and_then(Value::as_str)
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    format!(
                        "{}.{}",
                        extension.resource.0,
                        lifecycle_name(extension.lifecycle)
                    )
                });
            let operation_id = OperationId(operation_id);

            let resource = resources.get_mut(&extension.resource).ok_or_else(|| {
                ir_error(format!(
                    "operation {method_name} {path} references unknown resource {}",
                    extension.resource.0
                ))
            })?;

            set_lifecycle(
                &mut resource.lifecycle,
                extension.lifecycle,
                operation_id.clone(),
                &extension.resource,
            )?;

            let operation = OperationIr {
                method: parse_method(method_name)?,
                path: path.clone(),
                path_parameters: extension.path_parameters,
                request_schema: None,
                response_schema: None,
            };

            if operations.insert(operation_id.clone(), operation).is_some() {
                return Err(ir_error(format!(
                    "duplicate operation id {}",
                    operation_id.0
                )));
            }
        }
    }

    Ok(operations)
}

fn set_lifecycle(
    draft: &mut LifecycleDraft,
    action: LifecycleAction,
    operation: OperationId,
    resource: &ResourceName,
) -> Result<(), LoaderError> {
    let slot = match action {
        LifecycleAction::Observe => &mut draft.observe,
        LifecycleAction::Create => &mut draft.create,
        LifecycleAction::Update => &mut draft.update,
        LifecycleAction::Delete => &mut draft.delete,
    };

    if slot.replace(operation).is_some() {
        return Err(ir_error(format!(
            "resource {} has duplicate {} operation",
            resource.0,
            lifecycle_name(action)
        )));
    }

    Ok(())
}

fn http_operations(path_item: &Map<String, Value>) -> impl Iterator<Item = (&str, &Value)> {
    const METHODS: [&str; 5] = ["get", "post", "put", "patch", "delete"];
    METHODS
        .into_iter()
        .filter_map(|method| path_item.get(method).map(|value| (method, value)))
}

fn parse_method(method: &str) -> Result<HttpMethod, LoaderError> {
    match method {
        "get" => Ok(HttpMethod::GET),
        "post" => Ok(HttpMethod::POST),
        "put" => Ok(HttpMethod::PUT),
        "patch" => Ok(HttpMethod::PATCH),
        "delete" => Ok(HttpMethod::DELETE),
        _ => Err(ir_error(format!("unsupported HTTP method {method}"))),
    }
}

fn lifecycle_name(action: LifecycleAction) -> &'static str {
    match action {
        LifecycleAction::Observe => "observe",
        LifecycleAction::Create => "create",
        LifecycleAction::Update => "update",
        LifecycleAction::Delete => "delete",
    }
}

fn object_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Map<String, Value>, LoaderError> {
    value
        .pointer(pointer)
        .and_then(Value::as_object)
        .ok_or_else(|| ir_error(format!("missing object at {pointer}")))
}

fn ir_error(message: impl Into<String>) -> LoaderError {
    LoaderError::Ir(message.into())
}

pub fn extract_schemas(openapi: &Value) -> Result<BTreeMap<SchemaName, SchemaIr>, LoaderError> {
    let schemas = openapi
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| ir_error("missing components.schemas"))?;

    schemas
        .iter()
        .map(|(name, value)| {
            let name = SchemaName(name.clone());
            let schema = extract_schema(value, &format!("components.schemas.{}", name.0))?;

            Ok((name, schema))
        })
        .collect()
}

fn extract_schema(value: &Value, location: &str) -> Result<SchemaIr, LoaderError> {
    if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
        return Ok(SchemaIr::Ref {
            target: extract_reference(reference, location)?,
        });
    }

    if let Some(variants) = value.get("oneOf") {
        return Ok(SchemaIr::OneOf {
            variants: extract_variants(variants, &format!("{location}.oneOf"))?,
        });
    }

    if let Some(variants) = value.get("allOf") {
        return Ok(SchemaIr::AllOf {
            variants: extract_variants(variants, &format!("{location}.allOf"))?,
        });
    }

    match value.get("type").and_then(Value::as_str) {
        Some("object") => extract_object(value, location),

        Some("array") => {
            let items = value
                .get("items")
                .ok_or_else(|| ir_error(format!("{location}: array has no items")))?;

            Ok(SchemaIr::Array {
                items: Box::new(extract_schema(items, &format!("{location}.items"))?),
            })
        }

        Some("string") => {
            let enum_values = value
                .get("enum")
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default();

            Ok(SchemaIr::String {
                format: optional_string(value, "format"),
                enum_values,
            })
        }

        Some("integer") => Ok(SchemaIr::Integer {
            format: optional_string(value, "format"),
        }),

        Some("number") => Ok(SchemaIr::Number {
            format: optional_string(value, "format"),
        }),

        Some("boolean") => Ok(SchemaIr::Boolean),

        Some(schema_type) => Err(ir_error(format!(
            "{location}: unsupported schema type {schema_type}"
        ))),

        None if value.get("properties").is_some()
            || value.get("additionalProperties").is_some() =>
        {
            extract_object(value, location)
        }

        None => Ok(SchemaIr::Any),
    }
}

fn extract_object(value: &Value, location: &str) -> Result<SchemaIr, LoaderError> {
    let required = value
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>();

    let mut properties = BTreeMap::new();

    if let Some(source_properties) = value.get("properties").and_then(Value::as_object) {
        for (name, property) in source_properties {
            properties.insert(
                name.clone(),
                FieldIr {
                    schema: extract_schema(property, &format!("{location}.properties.{name}"))?,
                    required: required.contains(name.as_str()),
                    nullable: property
                        .get("nullable")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    read_only: property
                        .get("readOnly")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),

                    write_only: property
                        .get("writeOnly")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),

                    ignored: property
                        .pointer("/x-kube-field/ignored")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                    description: optional_string(property, "description"),
                    default: property.get("default").cloned(),
                },
            );
        }
    }

    let additional_properties = match value.get("additionalProperties") {
        None | Some(Value::Bool(true)) => AdditionalPropertiesIr::Any,

        Some(Value::Bool(false)) => AdditionalPropertiesIr::Forbidden,

        Some(schema) if schema.is_object() => AdditionalPropertiesIr::Typed {
            schema: Box::new(extract_schema(
                schema,
                &format!("{location}.additionalProperties"),
            )?),
        },

        _ => {
            return Err(ir_error(format!(
                "{location}: invalid additionalProperties"
            )));
        }
    };

    Ok(SchemaIr::Object {
        properties,
        additional_properties,
    })
}

fn extract_variants(value: &Value, location: &str) -> Result<Vec<SchemaIr>, LoaderError> {
    value
        .as_array()
        .ok_or_else(|| ir_error(format!("{location} must be an array")))?
        .iter()
        .enumerate()
        .map(|(index, schema)| extract_schema(schema, &format!("{location}[{index}]")))
        .collect()
}

fn extract_reference(reference: &str, location: &str) -> Result<SchemaName, LoaderError> {
    let name = reference
        .strip_prefix("#/components/schemas/")
        .ok_or_else(|| {
            ir_error(format!(
                "{location}: external reference is unsupported: {reference}"
            ))
        })?;

    Ok(SchemaName(name.replace("~1", "/").replace("~0", "~")))
}

fn optional_string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}
