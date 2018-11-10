#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckMessage {
    pub name: String,
    pub command: String,
    pub timeout: u16,
    pub subscribers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCheckResultMessage {
    pub name: String,
    pub timestamp: String, // TODO:  Move to datetime object.
    pub status: CheckResultStatus,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CheckResultStatus {
    CRITICAL,
    WARNING,
    OK,
    UNKNOWN,
}

