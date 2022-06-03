pub struct KeyType {
    data_type: &'static str,
    resolved_value: Option<&'static str>,
    key_name: &'static str,
}

impl KeyType {
    pub fn set_value(&mut self, value: &'static str) -> () {
        self.resolved_value = Some(value);
    }

    pub fn get_value(&self) -> Option<&'static str> {
        self.resolved_value
    }

    pub fn get_data_type(&self) -> &'static str {
       self.data_type
    }
}

pub struct Config {
    pub keys: [KeyType; 2],
}

impl Config {
    pub fn new() -> Self {
        Config {
            keys: [
                KeyType {
                    data_type: "string",
                    resolved_value: None,
                    key_name: "proxy_address"
                },
                KeyType {
                    data_type: "string",
                    resolved_value: None,
                    key_name: "admin_address"
                }
            ]
        }
    }
}