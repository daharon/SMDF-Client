use clap::{Arg, App, ArgMatches};
use clap::{crate_version, crate_name, value_t_or_exit};
use rusoto_core::Region;

use std::str::FromStr;


#[derive(Clone, Debug)]
pub struct Config {
    pub client_name: String,
    pub tags: Vec<String>,
    pub region: Region,
    pub registration_parameter: String,
    pub deregistration_parameter: String,
    pub auto_deregister: bool,
    pub concurrency: usize,
    pub log_level: log::LevelFilter,
}

impl Config {
    pub fn new() -> Self {
        let matches = parse();
        let environ = matches.value_of("environment").unwrap();
        let registration_parameter = format!("/{}/smdf/registration", environ);
        let deregistration_parameter = format!("/{}/smdf/de-registration", environ);
        Self {
            client_name: matches.value_of("name").unwrap().to_string(),
            tags: matches.values_of("tags").unwrap().map(String::from).collect(),
            region: Region::from_str(matches.value_of("region").unwrap()).unwrap(),
            registration_parameter,
            deregistration_parameter,
            auto_deregister: matches.is_present("auto-deregister"),
            concurrency: value_t_or_exit!(matches.value_of("concurrency"), usize),
            log_level: value_t_or_exit!(matches.value_of("log-level"), log::LevelFilter),
        }
    }
}

fn parse() -> ArgMatches<'static> {
    App::new(crate_name!())
        .about("SMDF client.")
        .version(crate_version!())
        .arg(Arg::with_name("log-level")
            .short("l")
            .long("log-level")
            .help("Log level (TRACE, DEBUG, ERROR, WARN, INFO).")
            .required(false)
            .takes_value(true)
            .default_value("info")
            .value_name("LEVEL"))
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
            .help("The environment this monitoring client is running under.\nParameter store path /<env>/monitoring/registration will be used.")
            .required(true)
            .takes_value(true)
            .value_name("ENV"))
        .arg(Arg::with_name("concurrency")
            .short("c")
            .long("concurrency")
            .help("The maximum number of checks to run concurrently (1-256).\nNote:  Currently not used.")
            .required(false)
            .takes_value(true)
            .default_value("10")
            .value_name("INT"))
        .arg(Arg::with_name("auto-deregister")
            .long("auto-deregister")
            .help("Automatically de-register/de-activate the client on termination.")
            .required(false))
        .get_matches()
}
