use super::{
    CustomData, IdToken, IdTokenInfo, MessageContent, MeterValue, Transaction,
    TransactionEventEnum, TransactionLimit, TriggerReasonEnum, EVSE,
};
use crate::v2_1::{
    datatypes::CostDetailsType as CostDetails,
    enumerations::PreconditioningStatusEnumType as PreconditioningStatusEnum,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Transaction event sent by a Charging Station to the CSMS.
#[derive(Serialize, Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomData>,
    pub event_type: TransactionEventEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1), nested)]
    pub meter_value: Option<Vec<MeterValue>>,
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnum,
    #[validate(range(min = 0))]
    pub seq_no: i32,
    #[validate(nested)]
    pub transaction_info: Transaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 3))]
    pub number_of_phases_used: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub reservation_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditioning_status: Option<PreconditioningStatusEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_sleep: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token: Option<IdToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub cost_details: Option<CostDetails>,
}

/// Response to a TransactionEvent request.
#[derive(Serialize, Deserialize, Validate, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub id_token_info: Option<IdTokenInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub transaction_limit: Option<TransactionLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub updated_personal_message: Option<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1), nested)]
    pub updated_personal_message_extra: Option<Vec<MessageContent>>,
}

impl TransactionEventRequest {
    pub fn new(
        event_type: TransactionEventEnum,
        meter_value: Vec<MeterValue>,
        timestamp: DateTime<Utc>,
        trigger_reason: TriggerReasonEnum,
        seq_no: i32,
        transaction_info: Transaction,
    ) -> Self {
        Self {
            custom_data: None,
            event_type,
            meter_value: Some(meter_value),
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            preconditioning_status: None,
            evse_sleep: None,
            evse: None,
            id_token: None,
            cost_details: None,
        }
    }
}

impl Default for TransactionEventResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionEventResponse {
    pub fn new() -> Self {
        Self {
            custom_data: None,
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            transaction_limit: None,
            updated_personal_message: None,
            updated_personal_message_extra: None,
        }
    }
}
