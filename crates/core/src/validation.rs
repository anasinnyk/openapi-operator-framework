use std::collections::{BTreeMap, BTreeSet};

use crate::ir::{
    CredentialSourceIr, OperationId, OperationIr, ProviderIr, ResourceIr, ResourceName, ValueExpr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ValidationError {
    pub location: String,
    pub message: String,
}

pub fn validate(ir: &ProviderIr) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut kinds = BTreeMap::new();

    for (resource_name, resource) in &ir.resources {
        validate_unique_kinds(resource_name, resource, &mut kinds, &mut errors);
        validate_references(ir, resource_name, resource, &mut errors);
        validate_credentials(ir, resource_name, resource, &mut errors);

        let lifecycle = [
            ("observe", Some(&resource.lifecycle.observe)),
            ("create", resource.lifecycle.create.as_ref()),
            ("update", resource.lifecycle.update.as_ref()),
            ("delete", resource.lifecycle.delete.as_ref()),
        ];

        for (action, operation_id) in lifecycle {
            if let Some(operation_id) = operation_id {
                validate_operation(ir, resource_name, action, operation_id, &mut errors);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_unique_kinds(
    name: &ResourceName,
    resource: &ResourceIr,
    kinds: &mut BTreeMap<(String, String), ResourceName>,
    errors: &mut Vec<ValidationError>,
) {
    let key = (resource.api_version.clone(), resource.kind.clone());

    if let Some(previouse) = kinds.insert(key, name.clone()) {
        errors.push(ValidationError {
            location: format!("resource.{}", name.0),
            message: format!("apiVersion/kind already used by resource {}", previouse.0),
        });
    }
}

fn validate_references(
    ir: &ProviderIr,
    name: &ResourceName,
    resource: &ResourceIr,
    errors: &mut Vec<ValidationError>,
) {
    for (reference_name, reference) in &resource.references {
        if !ir.resources.contains_key(&reference.target) {
            errors.push(ValidationError {
                location: format!("resources.{}.references.{reference_name}", name.0),
                message: format!("unknown target resource {}", reference.target.0),
            });
        }
    }
}

fn validate_credentials(
    ir: &ProviderIr,
    name: &ResourceName,
    resource: &ResourceIr,
    errors: &mut Vec<ValidationError>,
) {
    let Some(credentials) = &resource.credentials else {
        return;
    };

    match &credentials.source {
        CredentialSourceIr::SecretKeySelector { path } => {
            if path.0.is_empty() {
                errors.push(ValidationError {
                    location: format!("resources.{}.credentials", name.0),
                    message: "secret path cannot be empty".into(),
                });
            }
        }
        CredentialSourceIr::Related { via } => match resolve_relation(ir, name, via) {
            Ok(target) => {
                let has_direct_credentials = target.credentials.as_ref().is_some_and(|creds| {
                    matches!(creds.source, CredentialSourceIr::SecretKeySelector { .. })
                });

                if !has_direct_credentials {
                    errors.push(ValidationError {
                        location: format!("resources.{}.credentials", name.0),
                        message: "credential relation must end at a resource with direct Secret credentials".into(),
                    });
                }
            }
            Err(message) => errors.push(ValidationError {
                location: format!("resources.{}.credentials", name.0),
                message,
            }),
        },
    }
}

fn resolve_relation<'a>(
    ir: &'a ProviderIr,
    start: &ResourceName,
    via: &[String],
) -> Result<&'a ResourceIr, String> {
    let mut current = ir
        .resources
        .get(start)
        .ok_or_else(|| format!("unknown resource {}", start.0))?;

    for relation_name in via {
        let relation = current
            .references
            .get(relation_name)
            .ok_or_else(|| format!("unknown relation {relation_name}"))?;

        current = ir
            .resources
            .get(&relation.target)
            .ok_or_else(|| format!("unknown resource {}", relation.target.0))?;
    }

    Ok(current)
}

fn validate_operation(
    ir: &ProviderIr,
    resource_name: &ResourceName,
    action: &str,
    operation_id: &OperationId,
    errors: &mut Vec<ValidationError>,
) {
    let location = format!("resources.{}.lifecycle.{action}", resource_name.0);

    let Some(operation) = ir.operations.get(operation_id) else {
        errors.push(ValidationError {
            location,
            message: format!("unknown operation {}", operation_id.0),
        });
        return;
    };

    validate_path_parameters(operation_id, operation, errors);

    for (parameter, expression) in &operation.path_parameters {
        validate_expression(
            ir,
            resource_name,
            expression,
            &format!("operations.{}.path_parameters.{parameter}", operation_id.0),
            errors,
        );
    }
}

fn validate_path_parameters(
    id: &OperationId,
    operation: &OperationIr,
    errors: &mut Vec<ValidationError>,
) {
    let expected = placeholders(&operation.path);
    let actual = operation
        .path_parameters
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    if expected != actual {
        errors.push(ValidationError {
            location: format!("operations.{}.path_parameters", id.0),
            message: format!("path expects {expected:?}, but bindings are {actual:?}"),
        });
    }
}

fn validate_expression(
    ir: &ProviderIr,
    resource: &ResourceName,
    expression: &ValueExpr,
    location: &str,
    errors: &mut Vec<ValidationError>,
) {
    match expression {
        ValueExpr::RelatedIdentifier { via, name } => match resolve_relation(ir, resource, via) {
            Ok(target) if !target.identifiers.contains_key(name) => {
                errors.push(ValidationError {
                    location: location.into(),
                    message: format!("target resource has no identifier {name}"),
                });
            }
            Err(message) => errors.push(ValidationError {
                location: location.into(),
                message,
            }),
            _ => {}
        },

        ValueExpr::Coalesce { values } => {
            if values.is_empty() {
                errors.push(ValidationError {
                    location: location.into(),
                    message: "coalesce cannot be empty".into(),
                });
            }

            for value in values {
                validate_expression(ir, resource, value, location, errors);
            }
        }

        ValueExpr::Field { .. } | ValueExpr::Literal { .. } => {}
    }
}

fn placeholders(path: &str) -> BTreeSet<&str> {
    path.split('{')
        .skip(1)
        .filter_map(|part| part.split_once('}'))
        .map(|(name, _)| name)
        .collect()
}
