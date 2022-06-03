#[macro_export]
macro_rules! into_service {
    ($router:expr) => {
        make_service_fn(
            move |_: &AddrStream| {
                let router_clone = $router.clone();

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
            }
        )

    }
}

#[macro_export]
macro_rules! into_server {
    ($address:expr,$router:expr) => {
        Server::new(
            HyperServer::bind(&$address).serve(
                make_service_fn(
                    move |_: &AddrStream| {
                        let router_clone = $router.clone();

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
                    }
                )
            )
        )
    }
}