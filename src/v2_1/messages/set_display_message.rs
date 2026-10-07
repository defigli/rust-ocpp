use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, MessageInfoType, StatusInfoType},
    enumerations::DisplayMessageStatusEnumType,
};

/// Configure a message for display by the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest {
    /// Required. Message to configure for display.
    #[validate(nested)]
    pub message: MessageInfoType,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Result of a SetDisplayMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageResponse {
    /// Required. Whether the message can be displayed.
    pub status: DisplayMessageStatusEnumType,

    /// Optional. Additional status details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
