extern crate clap;
extern crate reqwest;

mod config;

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

    let config: config::Config = match config::read(args.value_of("config")) {
        Ok(config) => config,
        Err(error) => panic!("Unable to parse config file {}: {}", config_path, error.description())
    };

    let app_key = args.value_of("app").unwrap();

    let app = match config.apps.get(app_key) {
        Some(app) => app,
        None      => panic!("App not found in config: {}", app_key)
    };

    let environment_key = args.value_of("environment").unwrap();

    if !app.environments.contains_key(environment_key) {
        panic!("Application {} does not contain environment {}", app_key, environment_key);
    }

    let urls = match app.urls_for_environment(environment_key) {
        Some(urls) => urls,
        None => panic!("Unable to get endpoints for environment {} in {}", environment_key, app_key)
    };

    let client = match reqwest::Client::new() {
        Ok(client) => client,
        Err(error) => panic!("Failed to create HTTP client: {}", error.description())
    };

    let (tx, rx) = mpsc::channel();

    for url in urls.iter() {
        let tx = tx.clone();

        thread::spawn(move || {
            match client.get(url).send() {
                Ok(response) => tx.send(Ok(response)).unwrap(),
                Err(error) => tx.send(Err(error)).unwrap()
            };
        });
    }

    for url in urls.iter() {
        match rx.recv().unwrap() {
            Ok(response) => println!("{} {:?}", url, response.status()),
            Err(error)   => panic!("Failed to GET {}: {}", url, error.description())
        }
    }
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
