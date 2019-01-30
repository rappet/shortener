use crate::state::State;
use hyper::{Body, Request, Response, StatusCode};

use std::sync::{Arc, RwLock};

use crate::errors::{ErrorKind, Result};

pub struct Router<'a> {
    request: &'a Request<Body>,
    state: &'a Arc<RwLock<State>>,
}

impl<'a> Router<'a> {
    pub fn new(request: &'a hyper::Request<Body>, state: &'a Arc<RwLock<State>>) -> Self {
        Router { request, state }
    }

    pub fn serve(&self) -> Response<Body> {
        info!("Request for {}", self.request.uri());
        self.route().unwrap_or_else(|err| {
            error!("{}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        })
    }

    fn route(&self) -> Result<Response<Body>> {
        let key = self
            .request
            .uri()
            .path()
            .split('/')
            .nth(1)
            .ok_or(ErrorKind::UrlError)?;
        info!("Searching an entry for: {}", key);
        match key {
            "" => Ok(Response::new(Body::from(include_str!(
                "templates/index.html"
            )))),
            key => {
                if let Some(entry) = self.state.read().unwrap().find_mapping(key) {
                    //info!("Found mapping: {} => {}", key, entry.destination);
                    Ok(entry.generate_response())
                } else {
                    Ok(Response::new(Body::from("Did not find mapping.")))
                }
            }
        }
    }
}
