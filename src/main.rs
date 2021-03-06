#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate error_chain;

extern crate hyper;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Server, Uri};

use std::sync::Arc;
use std::sync::RwLock;

mod errors;
use errors::*;

mod state;
use state::State;

mod router;
use router::Router;

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

    let state = Arc::new(RwLock::new(State::new()));
    {
        let mut state = state.write().unwrap();
        state.add_mapping(
            "rppt",
            "https://example.rappet.de/".parse::<Uri>().unwrap().into(),
        );
        state.add_mapping(
            "permanent",
            state::Entry::new(
                "https://example.rappet.de/permanent".parse().unwrap(),
                state::RedirectType::HttpPermanent,
            ),
        );
        state.add_mapping(
            "meta",
            state::Entry::new(
                "https://example.rappet.de/meta".parse().unwrap(),
                state::RedirectType::HtmlMetaRefresh { seconds: 5 },
            ),
        );
    }

    let make_service = move || {
        let state = state.clone();
        service_fn_ok(move |request| Router::new(&request, &state).serve())
    };

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| eprintln!("server error: {}", e));
    info!("listening on {}", addr);

    info!("starting server...");
    hyper::rt::run(server);

    Ok(())
}
