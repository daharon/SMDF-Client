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
    pub concurrency: usize,
    pub log_level: log::LevelFilter,
}

impl Config {
    pub fn new() -> Self {
        let matches = parse();
        let registration_parameter: String = if matches.is_present("reg-parameter") {
            matches.value_of("reg-parameter").unwrap().to_string()
        } else {
            let environ = matches.value_of("environment").unwrap();
            format!("/{}/smdf/registration", environ)
        };
        Self {
            client_name: matches.value_of("name").unwrap().to_string(),
            tags: matches.values_of("tags").unwrap().map(String::from).collect(),
            region: Region::from_str(matches.value_of("region").unwrap()).unwrap(),
            registration_parameter,
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
        .arg(Arg::with_name("concurrency")
            .short("c")
            .long("concurrency")
            .help("The maximum number of checks to run concurrently (1-256).\nNote:  Currently not used.")
            .required(false)
            .takes_value(true)
            .default_value("10")
            .value_name("INT"))
        .get_matches()
}
