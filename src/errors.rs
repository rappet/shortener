error_chain! {
    foreign_links {
        AddrParse(std::net::AddrParseError);
    }

    errors {

    }
}
