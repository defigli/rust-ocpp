#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TriggerReasonEnumType {
    #[default]
    #[serde(rename = "Authorized")]
    Authorized,
    #[serde(rename = "CablePluggedIn")]
    CablePluggedIn,
    #[serde(rename = "ChargingRateChanged")]
    ChargingRateChanged,
    #[serde(rename = "ChargingStateChanged")]
    ChargingStateChanged,
    #[serde(rename = "CostLimitReached")]
    CostLimitReached,
    #[serde(rename = "Deauthorized")]
    Deauthorized,
    #[serde(rename = "EnergyLimitReached")]
    EnergyLimitReached,
    #[serde(rename = "EVCommunicationLost")]
    EVCommunicationLost,
    #[serde(rename = "EVConnectTimeout")]
    EVConnectTimeout,
    #[serde(rename = "LimitSet")]
    LimitSet,
    #[serde(rename = "MeterValueClock")]
    MeterValueClock,
    #[serde(rename = "MeterValuePeriodic")]
    MeterValuePeriodic,
    #[serde(rename = "OperationModeChanged")]
    OperationModeChanged,
    #[serde(rename = "TimeLimitReached")]
    TimeLimitReached,
    #[serde(rename = "Trigger")]
    Trigger,
    #[serde(rename = "UnlockCommand")]
    UnlockCommand,
    #[serde(rename = "StopAuthorized")]
    StopAuthorized,
    #[serde(rename = "EVDeparted")]
    EVDeparted,
    #[serde(rename = "EVDetected")]
    EVDetected,
    #[serde(rename = "RemoteStop")]
    RemoteStop,
    #[serde(rename = "RemoteStart")]
    RemoteStart,
    #[serde(rename = "AbnormalCondition")]
    AbnormalCondition,
    #[serde(rename = "SignedDataReceived")]
    SignedDataReceived,
    #[serde(rename = "ResetCommand")]
    ResetCommand,
    #[serde(rename = "RunningCost")]
    RunningCost,
    #[serde(rename = "SoCLimitReached")]
    SoCLimitReached,
    #[serde(rename = "TariffChanged")]
    TariffChanged,
    #[serde(rename = "TariffNotAccepted")]
    TariffNotAccepted,
    #[serde(rename = "TxResumed")]
    TxResumed,
}
