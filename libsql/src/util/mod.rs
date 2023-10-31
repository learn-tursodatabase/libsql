cfg_http! {
    pub mod box_clone_service;
    mod http;
    pub(crate) use self::http::{ConnectorService, Socket};
}

pub(crate) fn coerce_url_scheme(url: &str) -> String {
    let mut url = url.replace("libsql://", "https://");

    if !url.contains("://") {
        url = format!("https://{}", url)
    }

    url
}
