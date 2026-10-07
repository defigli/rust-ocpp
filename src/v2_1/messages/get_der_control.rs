use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{DERControlEnumType, DERControlStatusEnumType},
};

/// Request the Charging Station to report its DER control settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlRequest {
    /// Required. Request identifier returned in the corresponding report.
    pub request_id: i32,

    /// Optional. True requests default settings; false requests scheduled settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    /// Optional. Type of DER control to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<DERControlEnumType>,

    /// Optional. Id of the setting to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub control_id: Option<String>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Result of a GetDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlResponse {
    /// Required. Result of the request.
    pub status: DERControlStatusEnumType,

    /// Optional. Additional status details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
