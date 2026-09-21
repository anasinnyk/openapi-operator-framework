use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderIr {
    pub version: u32,
    pub provider: String,
    pub schemas: BTreeMap<SchemaName, SchemaIr>,
    pub resources: BTreeMap<ResourceName, ResourceIr>,
    pub operations: BTreeMap<OperationId, OperationIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceIr {
    pub api_version: String,
    pub kind: String,
    pub spec_schema: SchemaName,
    #[serde(default)]
    pub identifiers: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    pub references: BTreeMap<String, ReferenceIr>,
    pub credentials: Option<CredentialsIr>,
    pub lifecycle: LifecycleIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaRef {
    pub pointer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceIr {
    pub from: FieldPath,
    pub target: ResourceName,
    #[serde(default)]
    pub namespace: NamespacePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialsIr {
    pub source: CredentialsSourceIr,

    pub security_scheme: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CredentialsSourceIr {
    SecretKeySelector { path: FieldPath },
    Related { via: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleIr {
    pub observe: OperationId,
    pub create: Option<OperationId>,
    pub update: Option<OperationId>,
    pub delete: Option<OperationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationIr {
    pub method: HttpMethod,
    pub path: String,
    #[serde(default)]
    pub path_parameters: BTreeMap<String, ValueExpr>,
    pub request_schema: Option<SchemaRef>,
    pub response_schema: Option<SchemaRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ValueExpr {
    Field { path: FieldPath },
    RelatedIdentifier { via: Vec<String>, name: String },
    Coalesce { values: Vec<ValueExpr> },
    Literal { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FieldPath(pub String);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespacePolicy {
    #[default]
    SameAsResource,
    FromReference,
    ClusterScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SchemaIr {
    Ref {
        target: SchemaName,
    },

    Object {
        #[serde(default)]
        properties: BTreeMap<String, FieldIr>,

        #[serde(default)]
        additional_properties: AdditionalPropertiesIr,
    },

    Array {
        items: Box<SchemaIr>,
    },

    String {
        format: Option<String>,

        #[serde(default)]
        enum_values: Vec<String>,
    },

    Integer {
        format: Option<String>,
    },

    Number {
        format: Option<String>,
    },

    Boolean,

    OneOf {
        variants: Vec<SchemaIr>,
    },

    AllOf {
        variants: Vec<SchemaIr>,
    },

    Any,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldIr {
    pub schema: SchemaIr,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub nullable: bool,

    pub description: Option<String>,
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AdditionalPropertiesIr {
    #[default]
    Forbidden,

    Any,

    Typed {
        schema: Box<SchemaIr>,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_yaml;

    const FIXTURE: &str = include_str!("fixtures/minimal-provider.yaml");

    #[test]
    fn minimal_provider_fixture_describes_the_ir() {
        let ir: ProviderIr =
            serde_yaml::from_str(FIXTURE).expect("fixture must be valid ProviderIr");

        assert_eq!(ir.version, 1);
        assert_eq!(ir.provider, "cloudflare");
        assert_eq!(ir.resources.len(), 3);

        let dns_record = &ir.resources[&ResourceName("dns_record".into())];
        assert_eq!(dns_record.kind, "DNSRecord");
        assert_eq!(dns_record.references["zone"].target.0, "zone");
        assert_eq!(
            dns_record.credentials.as_ref().unwrap().via,
            vec!["zone".to_string(), "account".to_string()]
        );

        let get = &ir.operations[&OperationId("dns_record.get".into())];
        assert_eq!(get.method, HttpMethod::GET);
        assert_eq!(get.path_parameters.len(), 2);

        let yaml = serde_yaml::to_string(&ir).expect("ProviderIr must serialize");
        let decoded: ProviderIr =
            serde_yaml::from_str(&yaml).expect("serialized IR must deserialize");
        assert_eq!(decoded, ir);
    }
}
