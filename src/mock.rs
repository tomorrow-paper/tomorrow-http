use tomorrow_core::Result;
use super::Requester;

use serde_json;
use serde::de::DeserializeOwned;

pub struct MockClient {
    json: String
}

impl MockClient {

    pub fn with_json(json: &str) -> Self {
        MockClient {
            json: String::from(json)
        }
    }
}

impl Requester for MockClient {

    fn request<T>(&self, _: &str) -> Result<T> where T: DeserializeOwned {
        Ok(serde_json::from_str::<T>(&self.json)?)
    }
}

#[cfg(test)]
mod tests {

    use ::Requester;
    use ::mock::MockClient;

    #[derive(Deserialize)]
    pub struct Build {
        pub build_id: i32
    }

    #[test]
    fn mock_client_should_return_deserialized_structure() {
        let json = r#"{"build_id": 42}"#;
        let client = MockClient::with_json(json);
        let build = client.request::<Build>("build");

        assert!(build.is_ok());
        assert_eq!(build.unwrap().build_id, 42);
    }
}
