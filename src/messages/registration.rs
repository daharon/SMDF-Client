pub trait RegistrationRequest { }
pub trait RegistrationResponse { }

/// Client registration.
#[derive(Debug, Serialize)]
pub struct ClientRegistrationRequest {
    pub name: String,
    pub tags: Vec<String>,
}

impl RegistrationRequest for ClientRegistrationRequest { }

impl ClientRegistrationRequest {
    pub fn new<T>(name: &str, tags: &[T]) -> Self
    where T: AsRef<str>
    {
        Self {
            name: String::from(name),
            tags: tags.iter().map(|s| { String::from(s.as_ref()) }).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientRegistrationResponse {
    #[serde(rename = "commandQueue")]
    pub command_queue: String,
    #[serde(rename = "resultQueue")]
    pub result_queue: String,
}

impl RegistrationResponse for ClientRegistrationResponse { }

/// Client de-registration.
#[derive(Debug, Serialize)]
pub struct ClientDeregistrationRequest {
    pub name: String,
}

impl RegistrationRequest for ClientDeregistrationRequest { }

impl ClientDeregistrationRequest {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientDeregistrationResponse {
    pub code: i64,
    pub message: String,
}

impl RegistrationResponse for ClientDeregistrationResponse { }
