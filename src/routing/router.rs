use crate::routing::Route;
use crate::plugin::Plugin;
use crate::server::IncomingRequest;

use hyper::{Response, Body};

#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self {
        Self {
            routes: Vec::new(),
        }
    }
}

impl Router {
    pub fn test(&self) -> () {
        println!("HALLO")
    }

    pub fn explore_route(&self, incoming: IncomingRequest) -> Route {

        Route {

        }
    }

    pub fn load_routes(&self, routes: Vec<Route>) -> () {

    }

    pub fn load_plugins(&self, plugins: Vec<Plugin>) -> () {

    }

    pub fn handle_route(&self, route: Route) -> Response<Body> {
        Response::new("routed".into())
    }
}