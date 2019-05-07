use std::error::Error;

use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, LambdaClient, Lambda};
use serde::Serialize;
use serde::de::DeserializeOwned;


#[derive(Debug)]
pub struct RegistrationError {
    pub code: i64,
    pub description: String,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Response code:  {}\nError:  {}", self.code, self.description)
    }
}

impl Error for RegistrationError { }

/// Execute registration/de-registration requests.
pub trait RegistrationRequest: Serialize {
    type Response: DeserializeOwned;

    fn execute(&self, region: &Region, function: &str) -> Result<Self::Response, Box<dyn Error>> {
        let payload = serde_json::to_string(self).unwrap().as_bytes().to_vec();
        let invoke_request = InvocationRequest {
            client_context: None,
            function_name: String::from(function),
            invocation_type: Some(String::from("RequestResponse")),
            log_type: Some(String::from("Tail")),
            payload: Some(payload),
            qualifier: None
        };
        let client = LambdaClient::new(region.clone());
        let response = client.invoke(invoke_request).sync()?;
        if response.status_code.unwrap() == 200 {
            let registration_response = serde_json::from_slice::<Self::Response>(response.payload.unwrap().as_ref())?;
            Ok(registration_response)
        } else {
            let error = RegistrationError {
                code: response.status_code.unwrap(),
                description: response.log_result.unwrap(),
            };
            Err(Box::new(error))
        }
    }
}

pub mod registration;
pub mod deregistration;
