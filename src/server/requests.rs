use std::cmp::PartialEq;
use crate::routing::Route;
use hyper::{Request, Body};

pub struct IncomingRequest {}

impl PartialEq<Route> for IncomingRequest {
    fn eq(&self, other: &Route) -> bool {
        todo!()
    }
}

impl Into<IncomingRequest> for Request<Body> {
    fn into(self) -> IncomingRequest {
        IncomingRequest {

        }
    }
}