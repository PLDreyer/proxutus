use crate::logger::Logger;
use crate::routing::Route;
use crate::plugin::Plugin;

pub struct Database<'a> {
    logger: &'a Logger,
}

impl<'a> Database<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        Self {
            logger,
        }
    }

    pub fn connect(&self) -> &Self {
        self.logger.info("Connecting to database");
        &self
    }

    pub fn fetch_routes(&self) -> Vec<Route> {
        Vec::new()
    }

    pub fn fetch_plugins(&self) -> Vec<Plugin> {
        Vec::new()
    }
}