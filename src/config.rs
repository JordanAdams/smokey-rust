#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::env;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

#[derive(Deserialize, Debug)]
struct Application {
    name: String,
    environments: BTreeMap<String, String>,
    endpoints: Vec<String>
}

impl Application {
    fn urls_for_environment(&self, environment_key: &str) -> Option<Vec<String>> {
        let base_url = match self.environments.get(environment_key) {
            Some(base_url) => base_url,
            None => return None
        };

        let urls = self.endpoints
            .iter()
            .map(|endpoint| format!("{}{}", base_url, endpoint))
            .collect();

        return Some(urls);
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    apps: BTreeMap<String, Application>
}

enum ConfigReadError {
    std::io::Error
};


pub fn read(path: Option<&str>) -> Result<Config, ConfigReadError> {
    let config_path = path.unwrap_or(get_default_config_path());

    return File::open(config_path)
        .map(|file| {
            let mut yaml = String::new();
            file.read_to_string(&mut yaml);

            return yaml;
        })
        .map(|yaml| serde_yaml::from_str(&yaml) as Config);
}

fn get_default_config_path() {
    let home_dir = env::home_dir().unwrap();
    let config_path = Path::new(&home_dir).join(".smokey-config.yml");

    return config_path.to_str().unwrap();
}
