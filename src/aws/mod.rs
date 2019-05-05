use rusoto_core::Region;
use rusoto_lambda::{
    LambdaClient, Lambda, InvocationRequest
};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::messages::registration::{
    RegistrationRequest, RegistrationResponse
};

use std::error::Error;


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


/// Perform registration and de-registration requests.
pub fn client_registration<R, S>(region: &Region, function: &str, request: &R)
    -> Result<S, Box<dyn Error>>
    where
        R: RegistrationRequest + Serialize,
        S: RegistrationResponse + DeserializeOwned
{
    let payload = serde_json::to_string(request).unwrap().as_bytes().to_vec();
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
        let registration_response = serde_json::from_slice::<S>(response.payload.unwrap().as_ref())?;
        Ok(registration_response)
    } else {
        let error = RegistrationError {
            code: response.status_code.unwrap(),
            description: response.log_result.unwrap(),
        };
        Err(Box::new(error))
    }
}
