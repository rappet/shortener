use hyper::{Body, Response, StatusCode, Uri};

use std::collections::HashMap;

pub struct State {
    entries: HashMap<String, Entry>,
}

impl State {
    pub fn new() -> State {
        State {
            entries: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, key: &str, url: Entry) {
        self.entries.insert(key.to_owned(), url);
    }

    pub fn find_mapping(&self, key: &str) -> Option<&Entry> {
        self.entries.get(key)
    }
}

pub struct Entry {
    destination: Uri,
    redirect_type: RedirectType,
}

impl Entry {
    pub fn new(destination: Uri, redirect_type: RedirectType) -> Self {
        Entry {
            destination,
            redirect_type,
        }
    }

    pub fn generate_response(&self) -> Response<Body> {
        match self.redirect_type {
            RedirectType::HttpTemporary => Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header("Location", self.destination.clone().to_string())
                .body(Body::from("redirecting..."))
                .unwrap(),
            RedirectType::HttpPermanent => Response::builder()
                .status(StatusCode::PERMANENT_REDIRECT)
                .header("Location", self.destination.clone().to_string())
                .body(Body::from("redirecting..."))
                .unwrap(),
            RedirectType::HtmlMetaRefresh { seconds } => Response::builder()
                .body(Body::from(format!(
                    "\
                     <!DOCTYPE html><html>\
                     <head>\
                     <title>Redirecting...</title>\
                     <meta http-equiv=\"refresh\" content=\"{seconds}; URL={url}\">\
                     </head>\
                     <body>\
                     <p>Redirecting to <a href=\"{url}\">{url}</a> in {seconds} seconds.</p>\
                     </body>\
                     </html>\
                     ",
                    url = self.destination,
                    seconds = seconds
                )))
                .unwrap(),
        }
    }
}

impl From<Uri> for Entry {
    fn from(url: Uri) -> Self {
        Entry {
            destination: url,
            redirect_type: RedirectType::default(),
        }
    }
}

pub enum RedirectType {
    HttpTemporary,
    HttpPermanent,
    HtmlMetaRefresh { seconds: u32 },
}

impl Default for RedirectType {
    fn default() -> Self {
        RedirectType::HttpTemporary
    }
}
