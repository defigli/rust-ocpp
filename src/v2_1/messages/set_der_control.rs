use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        CustomDataType, DERCurveType, EnterServiceType, FixedPFType, FixedVarType, FreqDroopType,
        GradientType, LimitMaxDischargeType, StatusInfoType,
    },
    enumerations::{DERControlEnumType, DERControlStatusEnumType},
};

/// Configure DER control settings at the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlRequest {
    /// Required. True sets a default control; false sets a scheduled control.
    pub is_default: bool,

    /// Required. Identifier for this control setting.
    #[validate(length(max = 36))]
    pub control_id: String,

    /// Required. Type of DER control being configured.
    pub control_type: DERControlEnumType,

    /// Optional. DER curve settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub curve: Option<DERCurveType>,

    /// Optional. Enter-service settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub enter_service: Option<EnterServiceType>,

    /// Optional. Fixed power-factor absorption settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    #[serde(rename = "fixedPFAbsorb")]
    pub fixed_pf_absorb: Option<FixedPFType>,

    /// Optional. Fixed power-factor injection settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    #[serde(rename = "fixedPFInject")]
    pub fixed_pf_inject: Option<FixedPFType>,

    /// Optional. Fixed var settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub fixed_var: Option<FixedVarType>,

    /// Optional. Frequency-droop settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub freq_droop: Option<FreqDroopType>,

    /// Optional. Gradient settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub gradient: Option<GradientType>,

    /// Optional. Maximum-discharge limit settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub limit_max_discharge: Option<LimitMaxDischargeType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Result of a SetDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlResponse {
    /// Required. Result of setting the DER control.
    pub status: DERControlStatusEnumType,

    /// Optional. Additional status details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Ids of settings superseded by this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub superseded_ids: Option<Vec<String>>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
