#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckMessage {
    pub group: String,
    pub name: String,
    pub command: String,
    pub timeout: u16,
    pub subscribers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckResultMessage {
    pub group: String,
    pub name: String,
    pub client: String,
    pub timestamp: String, // TODO:  Move to datetime object.
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
