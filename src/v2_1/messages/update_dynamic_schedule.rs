use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingScheduleUpdateType, CustomDataType, StatusInfoType},
    enumerations::ChargingProfileStatusEnumType,
};

/// Provide an updated schedule for a dynamic charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDynamicScheduleRequest {
    /// Required. Id of the charging profile to update.
    pub charging_profile_id: i32,

    /// Required. Updated charging schedule values.
    #[validate(nested)]
    pub schedule_update: ChargingScheduleUpdateType,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Result of an UpdateDynamicSchedule request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDynamicScheduleResponse {
    /// Required. Result of the update.
    pub status: ChargingProfileStatusEnumType,

    /// Optional. Additional status details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Vendor-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}
