#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDetail {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation_detail::Properties>,
}
pub mod operation_detail {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "serviceSpecification", skip_serializing_if = "Option::is_none")]
        pub service_specification: Option<properties::ServiceSpecification>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ServiceSpecification {
            #[serde(rename = "metricSpecifications", skip_serializing)]
            pub metric_specifications: Vec<MetricSpecifications>,
            #[serde(rename = "logSpecifications", skip_serializing)]
            pub log_specifications: Vec<LogSpecifications>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSpecifications {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", skip_serializing)]
    pub display_description: Option<String>,
    #[serde(skip_serializing)]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", skip_serializing)]
    pub aggregation_type: Option<String>,
    #[serde(skip_serializing)]
    pub dimensions: Vec<MetricDimensions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSpecifications {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", skip_serializing)]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDimensions {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<OperationDetail>,
    #[serde(rename = "nextLink", skip_serializing)]
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
    pub location: String,
    pub sku: ResourceSku,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServers {
    pub value: Vec<AnalysisServicesServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServerUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AnalysisServicesServerMutableProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServerProperties {
    #[serde(flatten)]
    pub analysis_services_server_mutable_properties: AnalysisServicesServerMutableProperties,
    #[serde(skip_serializing)]
    pub state: Option<analysis_services_server_properties::State>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<analysis_services_server_properties::ProvisioningState>,
    #[serde(rename = "serverFullName", skip_serializing)]
    pub server_full_name: Option<String>,
}
pub mod analysis_services_server_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Deleting,
        Succeeded,
        Failed,
        Paused,
        Suspended,
        Provisioning,
        Updating,
        Suspending,
        Pausing,
        Resuming,
        Preparing,
        Scaling,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<resource_sku::Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod resource_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Development,
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisServicesServerMutableProperties {
    #[serde(rename = "asAdministrators", skip_serializing_if = "Option::is_none")]
    pub as_administrators: Option<ServerAdministrators>,
    #[serde(rename = "backupBlobContainerUri", skip_serializing_if = "Option::is_none")]
    pub backup_blob_container_uri: Option<String>,
    #[serde(rename = "managedMode", skip_serializing_if = "Option::is_none")]
    pub managed_mode: Option<analysis_services_server_mutable_properties::ManagedMode>,
    #[serde(rename = "serverMonitorMode", skip_serializing_if = "Option::is_none")]
    pub server_monitor_mode: Option<analysis_services_server_mutable_properties::ServerMonitorMode>,
}
pub mod analysis_services_server_mutable_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ManagedMode {}
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerMonitorMode {}
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerAdministrators {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckServerNameAvailabilityParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckServerNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuEnumerationForNewResourceResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuEnumerationForExistingResourceResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuDetailsForExistingResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuDetailsForExistingResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "subCode", skip_serializing_if = "Option::is_none")]
    pub sub_code: Option<i32>,
    #[serde(rename = "httpStatusCode", skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    #[serde(rename = "timeStamp", skip_serializing)]
    pub time_stamp: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}
