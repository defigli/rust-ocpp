#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TariffKindEnumType {
    #[default]
    #[serde(rename = "DefaultTariff")]
    DefaultTariff,
    #[serde(rename = "DriverTariff")]
    DriverTariff,
}
