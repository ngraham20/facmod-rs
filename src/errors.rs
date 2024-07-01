use error_chain::error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        Json(serde_json::Error);
        Yaml(serde_yaml::Error);
        HTTPRequest(reqwest::Error);
    }
}
