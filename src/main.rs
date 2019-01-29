#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server, StatusCode};

use std::sync::Arc;
use std::sync::RwLock;

mod errors;
use errors::*;

mod state;

fn main() {
    env_logger::init();

    // Error handling code.
    if let Err(ref e) = run() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let addr = "[::]:8080".parse()?;

    let state = Arc::new(RwLock::new(state::State::new()));
    {
        let mut state = state.write().unwrap();
        state.add_mapping(
            "rppt",
            state::Entry {
                destination: String::from("https://example.rappet.de/"),
            },
        );
    }

    let make_service = move || {
        let state = state.clone();
        service_fn_ok(move |req| {
            info!("Request for {}", req.uri());
            let key = req.uri().path().split('/').nth(1);
            match key {
                Some(v) => {
                    info!("Searching an entry for: {}", v);
                    if let Some(entry) = state.read().unwrap().find_mapping(v) {
                        info!("Found mapping: {} => {}", v, entry.destination);
                        let mut response = Response::builder();
                        response
                            .header("Location", entry.destination.clone())
                            .status(StatusCode::TEMPORARY_REDIRECT)
                            .body(Body::from("redirecting..."))
                            .unwrap()
                    } else {
                        Response::new(Body::from("Did not find mapping."))
                    }
                }
                None => Response::new(Body::from("The Uri is empty!")),
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| eprintln!("server error: {}", e));
    info!("listening on {}", addr);

    info!("starting server...");
    hyper::rt::run(server);

    Ok(())
}
