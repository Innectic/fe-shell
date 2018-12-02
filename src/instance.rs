use std::{
    env,
    io::{stdin, stdout, Write},
};

use config::{create_default_config, read_config_file, Configuration};

const CONFIGURATION_FILE: &str = ".shell.json";

#[derive(Debug)]
pub enum InstanceErrors {
    CouldNotFindHomeDir,
    CouldNotParseConfig,
}

pub struct Instance {
    config: Configuration,
}

impl Instance {
    pub fn new() -> Result<Instance, InstanceErrors> {
        match env::home_dir() {
            Some(path) => {
                let config_path = path.as_path().join(CONFIGURATION_FILE);
                if !config_path.exists() {
                    // Since the configuration does not exist, we're going to create a default one.
                    create_default_config(&config_path);
                }
                let mut config: Configuration =
                    read_config_file(&config_path).expect("could not parse config");
                let current_dir = get_current_directory();

                let prefix = config.prefix.clone().replace("<DIR>", &current_dir);
                config.prefix = prefix;

                Ok(Instance { config })
            }
            None => Err(InstanceErrors::CouldNotFindHomeDir),
        }
    }

    pub fn handle(&self) {
        loop {
            print!(
                "{}{} ",
                &self.config.prefix,
                self.config.shell_character.to_string()
            );
            stdout().flush();

            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    buffer = buffer.trim().to_string();
                    if buffer == "" {
                        continue;
                    }
                    println!("{}", buffer);
                }
                Err(err) => println!("Encountered an error while taking from stdin: {}", err),
            }
        }
    }
}

fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_string(),
        Err(error) => error.to_string(),
    }
}
