#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PublishFirmwareStatusEnumType {
    #[default]
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Published")]
    Published,
    #[serde(rename = "DownloadScheduled")]
    DownloadScheduled,
    #[serde(rename = "InvalidChecksum")]
    InvalidChecksum,
    #[serde(rename = "DownloadFailed")]
    DownloadFailed,
    #[serde(rename = "Downloaded")]
    Downloaded,
    #[serde(rename = "Downloading")]
    Downloading,
    #[serde(rename = "DownloadPaused")]
    DownloadPaused,
    #[serde(rename = "ChecksumVerified")]
    ChecksumVerified,
    #[serde(rename = "PublishFailed")]
    PublishFailed,
}
