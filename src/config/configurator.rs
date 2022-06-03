use std::collections::HashMap;
use crate::config::{ConfigLoader, EnvironmentLoader, keys::Config};
use crate::config::keys::KeyType;

pub(crate) struct Configurator {
    config: Config,
    keys: HashMap<&'static str, &'static str>,
    config_loader: ConfigLoader,
    environment_loader: EnvironmentLoader,
}

impl Configurator {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            keys: HashMap::new(),
            config_loader: ConfigLoader::new(),
            environment_loader: EnvironmentLoader::new(),
        }
    }

    pub fn load_state(&self) {
        for mut key_type in &self.config.keys {
            match key_type.get_data_type() {
                "string" => {
                    self.handle_string_type(key_type);
                },
                "int" => {
                    self.handle_int_type(key_type)
                }
                _ => {}
            }
        }
    }

    fn handle_string_type(&self, key_type: &KeyType) -> () {}

    fn handle_int_type(&self, key_type: &KeyType) -> () {}
}