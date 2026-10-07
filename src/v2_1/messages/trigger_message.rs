use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, EVSEType, StatusInfoType},
    enumerations::{MessageTriggerEnumType, TriggerMessageStatusEnumType},
};

/// Request the Charging Station to send a specified message.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    /// Optional. EVSE for which the message should be triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSEType>,

    /// Required. Type of message to trigger.
    pub requested_message: MessageTriggerEnumType,

    /// Optional. Message name used when requestedMessage is CustomTrigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub custom_trigger: Option<String>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Result of a TriggerMessage request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageResponse {
    /// Required. Whether the message trigger was accepted.
    pub status: TriggerMessageStatusEnumType,

    /// Optional. Additional status details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
