#![feature(future_join,future_poll_fn)]

mod logger;
mod routing;
mod server;
mod plugin;
mod database;
mod utils;

extern crate hyper;
extern crate diesel;

use std::clone::Clone;
use std::borrow::Borrow;
use std::convert::Infallible;
use std::error::Error;
use std::future::{join};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::char::ToLowercase;
use hyper::server::Server as HyperServer;
use hyper::{Response, Request, Body};
use hyper::server::conn::{Connecting, AddrStream};
use hyper::service::{service_fn, make_service_fn};
use logger::{Logger, LogLevel, Loggable};
use routing::{Router, Route};
use server::{Server, IncomingRequest};
use plugin::Plugin;
use database::Database;

struct OutgoingResponse {}
struct PipelineError {}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let logger = Logger::new(None);

    let database = Database::new(
        &logger
    );

    database.connect();

    let loaded_plugins = database.fetch_plugins();

    let routes = database.fetch_routes();

    let router_proxy = Router::default();
    let router_admin = Router::default();

    router_proxy.load_plugins(loaded_plugins);
    router_proxy.load_routes(routes);


    let proxy_router = Arc::new(
        Mutex::new(
            router_proxy
        )
    );

    let admin_router = Arc::new(
        Mutex::new(
            router_admin
        )
    );

    let proxy_address: SocketAddr = "127.0.0.1:8000".parse::<SocketAddr>().unwrap();
    let admin_address: SocketAddr = "127.0.0.1:8001".parse::<SocketAddr>().unwrap();

    let proxy_server = into_server!(&proxy_address, &proxy_router);
    let admin_server = into_server!(&admin_address, &admin_router);

    let run = || async {
        let (r_one, r_two) = join!(proxy_server.run(), admin_server.run()).await;

        r_one
    };

    run().await

    /*
    let service = into_service!(&router);


    let server = Server::new(
        HyperServer::bind(&address).serve(service)
    );
     */

    /*
    let server = Server::new(
        HyperServer::bind(&address).serve(make_service_fn(move |_| {
            let router_clone = router.clone();

            async move {
                Ok::<_, hyper::Error>(
                    service_fn(move |_req: Request<Body>| {
                        let locked_router = router_clone.lock().unwrap();
                        let incoming_request: IncomingRequest = _req.into();
                        let route = locked_router.explore_route(incoming_request);
                        let response = locked_router.handle_route(route);

                        async move {
                            Ok::<Response<Body>, Infallible>(
                                response
                                // Response::new("Test".into())
                            )
                        }
                    }))
            }
        }))
    );
     */

    // proxy_server.run().await
}
