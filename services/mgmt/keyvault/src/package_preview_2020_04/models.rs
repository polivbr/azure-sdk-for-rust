#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub family: sku::Family,
    pub name: sku::Name,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        A,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "standard")]
        Standard,
        #[serde(rename = "premium")]
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicyEntry {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    pub permissions: Permissions,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub sku: Sku,
    #[serde(rename = "accessPolicies", skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[serde(rename = "vaultUri", skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[serde(rename = "enabledForDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[serde(rename = "enabledForDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[serde(rename = "enabledForTemplateDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[serde(rename = "enableRbacAuthorization", skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_properties::CreateMode>,
    #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<vault_properties::ProvisioningState>,
    #[serde(rename = "privateEndpointConnections", skip_serializing)]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionItem>,
}
pub mod vault_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        RegisteringDns,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultPatchProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "accessPolicies", skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,
    #[serde(rename = "enabledForDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,
    #[serde(rename = "enabledForDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub enabled_for_disk_encryption: Option<bool>,
    #[serde(rename = "enabledForTemplateDeployment", skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,
    #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[serde(rename = "enableRbacAuthorization", skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,
    #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<vault_patch_properties::CreateMode>,
    #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[serde(rename = "networkAcls", skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}
pub mod vault_patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyProperties {
    #[serde(rename = "accessPolicies")]
    pub access_policies: Vec<AccessPolicyEntry>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedVaultProperties {
    #[serde(rename = "vaultId", skip_serializing)]
    pub vault_id: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(rename = "deletionDate", skip_serializing)]
    pub deletion_date: Option<String>,
    #[serde(rename = "scheduledPurgeDate", skip_serializing)]
    pub scheduled_purge_date: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "purgeProtectionEnabled", skip_serializing)]
    pub purge_protection_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCreateOrUpdateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: VaultProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultPatchParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultPatchProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultAccessPolicyParameters {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    pub properties: VaultAccessPolicyProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: VaultProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedVault {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedVaultProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedVaultListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeletedVault>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Resource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: vault_check_name_availability_parameters::Type,
}
pub mod vault_check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.KeyVault/vaults")]
        MicrosoftKeyVaultVaults,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing)]
    pub name_available: Option<bool>,
    #[serde(skip_serializing)]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        AccountNameInvalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<network_rule_set::Bypass>,
    #[serde(rename = "defaultAction", skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[serde(rename = "ipRules", skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[serde(rename = "virtualNetworkRules", skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Bypass {
        AzureServices,
        None,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    pub id: String,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    Disconnected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
    Disconnected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecification {
    #[serde(rename = "logSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[serde(rename = "metricSpecifications", skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "supportedAggregationTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[serde(rename = "supportedTimeGrainTypes", skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[serde(rename = "lockAggregationType", skip_serializing_if = "Option::is_none")]
    pub lock_aggregation_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<DimensionProperties>,
    #[serde(rename = "fillGapWithZero", skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[serde(rename = "internalMetricName", skip_serializing_if = "Option::is_none")]
    pub internal_metric_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "toBeExportedForShoebox", skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmSku {
    pub family: managed_hsm_sku::Family,
    pub name: managed_hsm_sku::Name,
}
pub mod managed_hsm_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        B,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_B1")]
        StandardB1,
        #[serde(rename = "Custom_B32")]
        CustomB32,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmProperties {
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "initialAdminObjectIds", skip_serializing_if = "Vec::is_empty")]
    pub initial_admin_object_ids: Vec<String>,
    #[serde(rename = "hsmUri", skip_serializing)]
    pub hsm_uri: Option<String>,
    #[serde(rename = "enableSoftDelete", skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,
    #[serde(rename = "softDeleteRetentionInDays", skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,
    #[serde(rename = "enablePurgeProtection", skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,
    #[serde(rename = "createMode", skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<managed_hsm_properties::CreateMode>,
    #[serde(rename = "statusMessage", skip_serializing)]
    pub status_message: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<managed_hsm_properties::ProvisioningState>,
}
pub mod managed_hsm_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateMode {
        #[serde(rename = "recover")]
        Recover,
        #[serde(rename = "default")]
        Default,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Provisioning,
        Failed,
        Updating,
        Deleting,
        Activated,
        SecurityDomainRestore,
        Restoring,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsm {
    #[serde(flatten)]
    pub managed_hsm_resource: ManagedHsmResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedHsmProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ManagedHsmSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedHsm>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedHsmError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Error>,
}
