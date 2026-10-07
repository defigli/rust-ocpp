use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, IdTokenType},
    enumerations::TariffKindEnumType,
    helpers::{datetime_rfc3339, validator::validate_identifier_string},
};

/// Assignment of a tariff to EVSEs or id tokens.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    /// Required. Unique identifier of the tariff.
    #[validate(length(max = 60), custom(function = "validate_identifier_string"))]
    pub tariff_id: String,

    /// Required. Kind of tariff.
    pub tariff_kind: TariffKindEnumType,

    /// Optional. Time from which this tariff is valid.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub valid_from: Option<DateTime<Utc>>,

    /// Optional. EVSEs to which the tariff is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub evse_ids: Option<Vec<i32>>,

    /// Optional. Id tokens to which the tariff is assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1), nested)]
    pub id_tokens: Option<Vec<IdTokenType>>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
