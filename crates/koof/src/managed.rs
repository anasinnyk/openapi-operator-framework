use std::collections::BTreeSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "PascalCase")]
pub enum ManagementAction {
    Observe,
    Create,
    Update,
    Delete,
    LateInitialize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum DeletionPolicy {
    #[default]
    Delete,
    Orphan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ManagedResourceSpec {
    #[serde(rename = "managementPolicies", default = "default_management_policies")]
    pub management_policies: BTreeSet<ManagementAction>,

    #[serde(rename = "deletionPolicy", default)]
    pub deletion_policy: DeletionPolicy,
}

impl Default for ManagedResourceSpec {
    fn default() -> Self {
        Self {
            management_policies: default_management_policies(),
            deletion_policy: DeletionPolicy::Delete,
        }
    }
}

fn default_management_policies() -> BTreeSet<ManagementAction> {
    [
        ManagementAction::Observe,
        ManagementAction::Create,
        ManagementAction::Update,
        ManagementAction::Delete,
        ManagementAction::LateInitialize,
    ]
    .into_iter()
    .collect()
}
