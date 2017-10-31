use std::str::FromStr;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Client as HttpClient;

use hyper::header::{
    Headers,
    Connection, UserAgent,
    AcceptLanguage, qitem
};
use hyper::LanguageTag;

use super::Client;

pub struct Builder {
    client: HttpClient,
    headers: Headers,
    api_url: String
}

impl Builder {
    
    pub fn new(api_url: &str) -> Self {
        Builder {
            client: Builder::default_http_client(),
            headers: Builder::default_headers(),
            api_url: String::from(api_url)
        }
    }
    
    fn default_http_client() -> HttpClient {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        
        HttpClient::with_connector(connector)
    }
    
    fn default_headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set(UserAgent(format!("{}/{}", ::PACKAGE, ::VERSION)));
        
        Builder::set_lang(&mut headers, "en-US");
        
        headers
    }
    
    fn set_lang(headers: &mut Headers, lang: &str) {
        let lang_tag = LanguageTag::from_str(lang).unwrap();
        headers.set(AcceptLanguage(vec![qitem(lang_tag)]));
    }
    
    /// All of the endpoints which are local aware accept a language parameter.
    ///
    /// Add the `Accept-Language` HTTP header with the value of `<language>`.
    pub fn lang(&mut self, lang: &str) -> &mut Self {
        Builder::set_lang(&mut self.headers, lang);
        
        self
    }
}

impl Into<Client> for Builder {
    
    /// Consumes this `Builder` instance in order to create a fully-configured API `Client`.
    fn into(self) -> Client {
        Client::new(self.client, self.headers, self.api_url)
    }
}