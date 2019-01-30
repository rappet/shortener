use crate::state::State;
use hyper::{Body, Request, Response};

use std::sync::{Arc, RwLock};

pub struct Router<'a> {
    request: &'a Request<Body>,
    state: &'a Arc<RwLock<State>>,
}

impl<'a> Router<'a> {
    pub fn new(request: &'a hyper::Request<Body>, state: &'a Arc<RwLock<State>>) -> Self {
        Router { request, state }
    }

    pub fn route(&self) -> Response<Body> {
        info!("Request for {}", self.request.uri());
        let key = self.request.uri().path().split('/').nth(1);
        match key {
            Some(v) => {
                info!("Searching an entry for: {}", v);
                if let Some(entry) = self.state.read().unwrap().find_mapping(v) {
                    //info!("Found mapping: {} => {}", v, entry.destination);
                    entry.generate_response()
                } else {
                    Response::new(Body::from("Did not find mapping."))
                }
            }
            None => Response::new(Body::from("The Uri is empty!")),
        }
    }
}
