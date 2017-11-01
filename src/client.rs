use tomorrow_core::{Error, Result};
use super::Requester;

use std::io::Read;

use serde_json;
use serde::de::DeserializeOwned;

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

    fn request<T>(&self, endpoint: &str) -> Result<T> where T: DeserializeOwned {
        
        let url = format!("{}/{}", self.api_url, endpoint);
        let mut response = self.client
            .get(&url)
            .headers(self.headers.clone())
            .send()?;
    
        let mut body = String::new();
        response.read_to_string(&mut body)?;
    
        match response.status {
            StatusCode::Conflict |
            StatusCode::BadRequest |
            StatusCode::UnprocessableEntity |
            StatusCode::Unauthorized |
            StatusCode::NotFound |
            StatusCode::Forbidden => Err(Error::from(body)),
            _ => Ok(serde_json::from_str::<T>(&body)?)
        }
    }
}