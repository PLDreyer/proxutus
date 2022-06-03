use std::cmp::PartialEq;
use crate::server::IncomingRequest;

#[derive(Clone, Copy)]
pub struct Route {}

impl PartialEq<IncomingRequest> for Route {
    fn eq(&self, other: &IncomingRequest) -> bool {
        todo!()
    }
}