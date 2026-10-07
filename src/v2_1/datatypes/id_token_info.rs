use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, IdTokenType, MessageContentType},
    enumerations::AuthorizationStatusEnumType,
    helpers::datetime_rfc3339,
};

/// Authorization status and related information for an id token.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    /// Required. Whether the identifier is allowed to charge.
    pub status: AuthorizationStatusEnumType,

    /// Optional. Expiration time for the authorization status.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,

    /// Optional. Business priority of this token, from -9 to 9.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -9, max = 9))]
    pub charging_priority: Option<i8>,

    /// Optional. Language code according to RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language1: Option<String>,

    /// Optional. Secondary language code according to RFC 5646.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))]
    pub language2: Option<String>,

    /// Optional. EVSE identifiers for which this token is valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub evse_id: Option<Vec<i32>>,

    /// Optional. Group token associated with this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub group_id_token: Option<IdTokenType>,

    /// Optional. Personal message to display for this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub personal_message: Option<MessageContentType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
