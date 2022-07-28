use crate::service::resources::{
    identity::DesiredCapability, identity::IdentityOperation, AuthenticationMechanism,
    DeviceCapabilities, Status,
};
use crate::service::responses::DeviceIdentityResponse;
use crate::service::{ServiceClient, API_VERSION};
use azure_core::error::{Error, ErrorKind};
use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::convert::TryInto;

azure_core::operation! {
    /// The CreateOrUpdateDeviceIdentityBuilder is used to construct a new device identity
    /// or the update an existing one.
    CreateOrUpdateDeviceIdentity,
    client: ServiceClient,
    operation: IdentityOperation,
    device_id: String,
    status: Status,
    authentication: AuthenticationMechanism,
    etag: Option<String>,
    ?capabilities: DeviceCapabilities
}

impl CreateOrUpdateDeviceIdentityBuilder {
    /// Sets a device capability on the device
    pub fn device_capability(mut self, desired_capability: DesiredCapability) -> Self {
        match desired_capability {
            DesiredCapability::IotEdge => {
                let caps = self
                    .capabilities
                    .get_or_insert(DeviceCapabilities::default());
                caps.iotedge = true;
            }
        }
        self
    }

    /// Performs the create or update request on the device identity
    pub fn into_future(self) -> CreateOrUpdateDeviceIdentity {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/{}?api-version={}",
                self.client.iot_hub_name, self.device_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&uri, Method::Put)?;

            if self.operation == IdentityOperation::Update {
                match &self.etag {
                    Some(etag) => {
                        request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
                    }
                    None => return Err(Error::message(ErrorKind::Other, "etag is not set")),
                }
            }

            let body = CreateOrUpdateDeviceIdentityBody {
                authentication: self.authentication,
                device_id: &self.device_id,
                status: self.status,
                capabilities: self.capabilities.unwrap_or_default(),
                etag: self.etag,
            };

            let body = azure_core::to_json(&body)?;
            request.set_body(body);

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await?
                .try_into()
        })
    }
}

pub type CreateOrUpdateDeviceIdentityResponse = DeviceIdentityResponse;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrUpdateDeviceIdentityBody<'a> {
    authentication: AuthenticationMechanism,
    device_id: &'a str,
    status: Status,
    capabilities: DeviceCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}
