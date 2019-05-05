use std::error::Error;

use rusoto_core::Region;
use rusoto_ssm::{
    SsmClient, Ssm, GetParameterRequest
};


pub fn get_registration_arn(region: &Region, parameter: &str) -> Result<String, Box<dyn Error>> {
    let ssm_client = SsmClient::new(region.clone());
    let req = GetParameterRequest {
        name: parameter.to_string(),
        with_decryption: None
    };
    let res = ssm_client.get_parameter(req).sync()?;
    Ok(res.parameter.unwrap().value.unwrap())
}
