/// Client de-registration.
#[derive(Debug, Serialize)]
pub struct Request {
    pub name: String,
}

impl super::RegistrationRequest for Request {
    type Response = Response;
}

impl Request {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub code: i64,
    pub message: String,
}
