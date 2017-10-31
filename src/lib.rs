extern crate tomorrow_core;

extern crate hyper;
extern crate hyper_native_tls;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub const PACKAGE: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod builder;
pub use self::builder::Builder;

mod client;
pub use self::client::Client;

mod requester;
pub use self::requester::Requester;

#[cfg(test)]
mod tests {

    use super::*;
    use tomorrow_core::Result;

    #[derive(Deserialize)]
    pub struct Build {
        pub build_id: i32
    }

    #[test]
    fn first_try() {
        let api_url = "https://api.guildwars2.com/v1";

        let builder: Builder = Builder::new(api_url);
        let client: Client = builder.into();

        let build: Result<Build> = client.request::<Build>("build");

        assert!(build.is_ok());
        assert!(build.unwrap().build_id > 0);
    }
}
