/// Client registration.
#[derive(Debug, Serialize)]
pub struct Request {
    pub name: String,
    pub tags: Vec<String>,
}

impl super::RegistrationRequest for Request {
    type Response = Response;
}

impl Request {
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
pub struct Response {
    #[serde(rename = "commandQueue")]
    pub command_queue: String,
    #[serde(rename = "resultQueue")]
    pub result_queue: String,
}
