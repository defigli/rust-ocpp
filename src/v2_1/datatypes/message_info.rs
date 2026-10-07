use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ComponentType, CustomDataType, MessageContentType},
    enumerations::{MessagePriorityEnumType, MessageStateEnumType},
    helpers::{datetime_rfc3339, validator::validate_identifier_string},
};

/// Message details used by display-message requests and notifications.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Required. Unique identifier for this message.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Required. Priority at which the message should be shown.
    pub priority: MessagePriorityEnumType,

    /// Optional. State during which the message should be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,

    /// Optional. Time at which the message should start being shown.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub start_date_time: Option<DateTime<Utc>>,

    /// Optional. Time at which the message should stop being shown.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub end_date_time: Option<DateTime<Utc>>,

    /// Optional. Transaction for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub transaction_id: Option<String>,

    /// Required. Message content.
    #[validate(nested)]
    pub message: MessageContentType,

    /// Optional. Display component to which this message applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub display: Option<ComponentType>,

    /// Optional. Additional message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4), nested)]
    pub message_extra: Option<Vec<MessageContentType>>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
