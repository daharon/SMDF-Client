use rusoto_ssm::{
    SsmClient, Ssm, GetParameterRequest
};

use crate::config::cli::Config;


pub fn get_registration_arn(config: &Config) -> String {
    let ssm_client = SsmClient::new(config.region.clone());
    let req = GetParameterRequest {
        name: config.registration_parameter.clone(),
        with_decryption: None
    };
    let res = ssm_client.get_parameter(req).sync();
    match res {
        Ok(param_result) => param_result.parameter.unwrap().value.unwrap(),
        Err(e) => {
            panic!("{}.\nNo value retrieved for registration parameter.\nCheck {} in the Parameter Store.",
                   e, config.registration_parameter)
        },
    }
}
