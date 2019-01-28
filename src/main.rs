#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::Future;
mod errors;
use errors::*;

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

    let make_service = || {
        service_fn_ok(|req| {
            info!("Request from {:?} for {}", req.headers(), req.uri());
            Response::new(Body::from("Hello, World!"))
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
