use hyper::Client as HttpClient;
use hyper::header::Headers;

use super::Requester;

pub struct Client {
    client: HttpClient,
    headers: Headers,
    api_url: String
}

impl Client {
    
    pub fn new(client: HttpClient, headers: Headers, api_url: String) -> Self {
        Client {
            client: client,
            headers: headers,
            api_url: api_url
        }
    }
}

impl Requester for Client {
    fn get_client(&self) -> &HttpClient {
        &self.client
    }
    
    fn get_headers(&self) -> Headers {
        self.headers.clone()
    }

    fn get_api_url(&self) -> String {
        self.api_url.clone()
    }
}