use tomorrow_core::{Error, Result};
use super::Requester;

use std::io::Read;
use hyper::Client as HttpClient;

use hyper::status::StatusCode;
use hyper::header::Headers;

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

    fn request(&self, endpoint: &str) -> Result<String> {
        
        let url = format!("{}/{}", self.api_url, endpoint);
        let mut response = self.client
            .get(&url)
            .headers(self.headers.clone())
            .send()?;

        let mut body = Vec::new();
        response.read_to_end(&mut body)?;

        let encoded = String::from_utf8_lossy(&*body);
        let raw_html = String::from(encoded);

        match response.status {
            StatusCode::Conflict |
            StatusCode::BadRequest |
            StatusCode::UnprocessableEntity |
            StatusCode::Unauthorized |
            StatusCode::NotFound |
            StatusCode::Forbidden => Err(Error::from(raw_html)),
            _ => Ok(raw_html)
        }
    }
}