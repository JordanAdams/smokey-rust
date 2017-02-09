// extern crate serde_yaml;
//
// use std::io::prelude::*;
// use std::fs::File;
// use std::error::Error;
// use std::path::Path;
// use std::collections::HashMap;
//
// struct Endpoint {
//     path: String
// }
//
// struct Environment {
//     url: String
// }
//
// struct Application {
//     id: String,
//     name: String,
//     environments: HashMap<String, Environment>,
//     endpoints: Vec<Endpoint>
// }
//
// impl Application {
//     fn from_yaml(id: &str, yaml: Yaml) -> Option<Application> {
//
//     }
//
//     // fn urls_for_environment(&self, key: &str) -> Vec<String> {
//     //     let environment = match self.environments.get(key) {
//     //         None => panic!("Failed to get environment {} from config", key),
//     //         Some(environment) => environment
//     //     };
//     //
//     //     self.endpoints
//     //         .iter()
//     //         .map(|endpoint| format!("{}{}", environment.url, endpoint.path))
//     //         .collect();
//     // }
// }
//
// pub fn get_application(key: &str, file_path: &str) -> Option<Application> {
//     let config = get_config(file_path);
//
//     return match config["apps"][key].as_hash() {
//         None =>
//             panic!("Application not found: {}", key),
//         Some(BTreeMap) =>
//             Application::from_yaml(key, application_yaml)
//     };
// }
//
// fn get_config(raw_path: &str) -> Yaml {
//     let path = Path::new(raw_path);
//
//     let mut file = match File::create(path) {
//         Err(err) => panic!("Unable to open config file {}: {}", raw_path, err.description()),
//         Ok(file) => file
//     };
//
//     let mut yaml = String::new();
//     file.read_to_string(&mut yaml);
//
//     return match YamlLoader::load_from_str(&yaml) {
//         Err(err) => panic!("Unable to parse config file {}: {:?}", raw_path, err),
//         Ok(config) => config[0]
//     };
// }
