use tomorrow_core::Result;

pub trait Requester {
    fn request(&self, endpoint: &str) -> Result<String>;
}