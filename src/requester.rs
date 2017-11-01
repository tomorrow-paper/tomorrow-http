use tomorrow_core::Result;
use serde::de::DeserializeOwned;

pub trait Requester {
    fn request<T>(&self, endpoint: &str) -> Result<T> where T: DeserializeOwned;
}