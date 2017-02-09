#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate clap;

use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Application {
    name: String,
    environments: BTreeMap<String, String>,
    endpoints: Vec<String>
}

impl Application {
    fn urls_for_environment(&self, environment_key: &str) -> Result<Vec<String>> {
        let environment = match self.environments.get(environment_key) {
            Some(environment) => environment,
            None => return None
        };
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    apps: BTreeMap<String, Application>
}

fn main() {
    let default_config_path = get_default_config_path();

    let args = clap::App::new("Smokey")
        .version("0.1.0")
        .author("Jordan Adams <hi@jordanadams.co.uk>")
        .about("Automated multi-environment smoke testing")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("PATH")
            .help("Sets a custom config file")
            .takes_value(true)
            .default_value(&default_config_path))
        .arg(clap::Arg::with_name("open")
            .short("o")
            .long("open")
            .help("Open URLs in default browser"))
        .arg(clap::Arg::with_name("app")
            .short("a")
            .long("app")
            .value_name("APP_NAME")
            .help("Sets the application to test")
            .takes_value(true)
            .default_value("default"))
        .arg(clap::Arg::with_name("environment")
            .index(1)
            .required(true)
            .help("Sets the environment to test"))
        .get_matches();

    let config_path = args.value_of("config").unwrap();

    let mut file = match File::open(config_path) {
        Ok(file)   => file,
        Err(error) => panic!("Failed to open config file {}: {}", config_path, error.description())
    };

    let mut yaml = String::new();
    match file.read_to_string(&mut yaml) {
        Ok(size)   => size,
        Err(error) => {
            println!("{:?}", error);
            panic!("Unable to read from config file {}: {}", config_path, error.description());
        }
    };

    let config: Config = match serde_yaml::from_str(&yaml) {
        Ok(config) => config,
        Err(error) => {
            println!("");
            panic!("Unable to parse config file {}: {}", config_path, error.description());
        }
    };

    let app_key = args.value_of("app").unwrap();

    let app = match config.apps.get(app_key) {
        Some(app) => app,
        None      => panic!("App not found in config: {}", app_key)
    };

    let environment_key = args.value_of("environment").unwrap();

    let urls = app.urls_for_environment(environment_key);

    println!("{:?}", app);
}

fn get_default_config_path() -> String {
    let home_dir = match std::env::home_dir() {
        Some(path) => path,
        None => panic!("Failed to determine a home directory")
    };

    let config_path = Path::new(&home_dir).join(".smokey-config.yml");

    return match config_path.to_str() {
        Some(path) => path.to_string(),
        None => panic!("Failed to generate a default config path")
    };
}
