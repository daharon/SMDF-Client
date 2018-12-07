use clap::{Arg, App, ArgMatches};
use clap::{crate_version, crate_name};
use rusoto_core::Region;

use std::str::FromStr;


#[derive(Clone, Debug)]
pub struct Config {
    pub client_name: String,
    pub tags: Vec<String>,
    pub region: Region,
    pub registration_parameter: String,
}

impl Config {
    pub fn new() -> Self {
        let matches = parse();
        let registration_parameter: String = if matches.is_present("reg-parameter") {
            matches.value_of("reg-parameter").unwrap().to_string()
        } else {
            let environ = matches.value_of("environment").unwrap();
            format!("/{}/monitoring/registration", environ)
        };
        Config {
            client_name: matches.value_of("name").unwrap().to_string(),
            tags: matches.values_of("tags").unwrap().map(String::from).collect(),
            region: Region::from_str(matches.value_of("region").unwrap()).unwrap(),
            registration_parameter,
        }
    }
}

fn parse() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about("Monitoring client.")
        .version(crate_version!())
        .arg(Arg::with_name("region")
            .short("r")
            .long("region")
            .help("AWS region.")
            .required(true)
            .takes_value(true)
            .value_name("REGION"))
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .help("The client-name to be registered with the monitoring backend.")
            .required(true)
            .takes_value(true)
            .value_name("NAME"))
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tags")
            .help("The check tags to run on this client.")
            .required(true)
            .takes_value(true)
            .multiple(true)
            .value_name("TAG,TAG,..."))
        .arg(Arg::with_name("environment")
            .short("e")
            .long("environment")
            .help("The environment this monitoring client is running under.\nNot used if `--reg-parameter` is set.\nParameter store path /<env>/monitoring/registration will be used.")
            .required_unless("reg-parameter")
            .takes_value(true)
            .value_name("ENV"))
        .arg(Arg::with_name("reg-parameter")
            .short("p")
            .long("reg-parameter")
            .help("Explicitly set the parameter store name to use.\nOverrides `--environment`.\neg. /dev/test/value")
            .required(false)
            .takes_value(true)
            .value_name("PATH"))
        .get_matches()
}
