use tomorrow_core::Result;
use super::Requester;

pub struct MockClient {
    content: String
}

impl MockClient {
    
    pub fn with_content(content: &str) -> Self {
        MockClient {
            content: String::from(content)
        }
    }
}

impl Requester for MockClient {

    fn request(&self, _: &str) -> Result<String> {
        Ok(self.content.clone())
    }
}

#[cfg(test)]
mod tests {

    use ::raw::Requester;
    use ::raw::mock::MockClient;

    #[test]
    fn mock_client_should_return_raw_content() {
        let html = r#"<!DOCTYPE html><html><head></head><body><h1>Hello World!</h1></body></html>"#;
        let client = MockClient::with_content(html);
        let content = client.request("_");

        assert!(content.is_ok());
        assert_eq!(content.unwrap(), html);
    }
}