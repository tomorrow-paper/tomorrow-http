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
    
    pub fn http(api_url: &str) -> Self {
        Builder::new(api_url, Builder::default_http_client())
    }

    pub fn https(api_url: &str) -> Self {
        Builder::new(api_url, Builder::default_https_client())
    }

    pub fn new(api_url: &str, client: HttpClient) -> Self {
        Builder {
            client: client,
            headers: Builder::default_headers(),
            api_url: String::from(api_url)
        }
    }
    
    fn default_http_client() -> HttpClient {
        HttpClient::new()
    }
    
    fn default_https_client() -> HttpClient {
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
    
    pub fn lang(mut self, lang: &str) -> Self {
        Builder::set_lang(&mut self.headers, lang);

        self
    }

    pub fn header(mut self, header: &'static str, value: &str) -> Self {
        let value = Vec::from(value.as_bytes());
        self.headers.set_raw(header, vec![value]);

        self
    }
}

impl Into<Client> for Builder {
    
    /// Consumes this `Builder` instance in order to create a fully-configured API `Client`.
    fn into(self) -> Client {
        Client::new(self.client, self.headers, self.api_url)
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn builder_should_contain_default_headers() {
        let builder = Builder::https("");
        let headers = builder.headers;

        let connection = headers.get_raw("Connection");
        let user_agent = headers.get_raw("User-Agent");
        let lang = headers.get_raw("Accept-Language");

        assert_eq!(headers.len(), 3);
        
        assert!(connection.is_some());
        assert_eq!(connection.unwrap()[0], b"close".to_vec());
        
        assert!(user_agent.is_some());
        assert_eq!(user_agent.unwrap()[0], Vec::from(format!("{}/{}", ::PACKAGE, ::VERSION).as_bytes()));
        
        assert!(lang.is_some());
        assert_eq!(lang.unwrap()[0], b"en-US".to_vec());
    }

    #[test]
    fn builder_header_should_add_custom_header() {
        let header = "X-Test-Header";
        let value = "test";

        let builder = Builder::https("").header(header, value);
        let test_header = builder.headers.get_raw(header);
        
        assert!(test_header.is_some());
        assert_eq!(test_header.unwrap()[0], Vec::from(value.as_bytes()));
    }
}