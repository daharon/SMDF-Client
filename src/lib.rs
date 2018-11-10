extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rusoto_core;
extern crate rusoto_sqs;
extern crate chrono;

mod messages;
pub mod consumer;
