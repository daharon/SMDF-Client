use chrono::{DateTime, Utc};


#[derive(Debug, Deserialize)]
pub struct ClientCheckMessage {
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: DateTime<Utc>,
    pub group: String,
    pub name: String,
    pub command: String,
    pub timeout: usize,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ClientCheckResultMessage {
    #[serde(rename = "completedAt")]
    pub completed_at: DateTime<Utc>,
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: DateTime<Utc>,
    #[serde(rename = "executedAt")]
    pub executed_at: DateTime<Utc>,
    pub group: String,
    pub name: String,
    pub source: String,
    pub status: CheckResultStatus,
    pub output: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CheckResultStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

impl CheckResultStatus {
    pub fn from_exit_code(code: i32) -> Self {
        match code {
            0 => CheckResultStatus::OK,
            1 => CheckResultStatus::WARNING,
            2 => CheckResultStatus::CRITICAL,
            _ => CheckResultStatus::UNKNOWN,
        }
    }
}
