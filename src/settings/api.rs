use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub struct ApiConfig {
    pub debug: bool,
    pub address: String,
    pub http_port: u16,
    pub https_port: u16,
}

lazy_static! {
  pub static ref data: Arc<RwLock<ApiConfig>> = Arc::new(RwLock::new(Config::builder()
  .add_source(config::File::with_name("./config/ApiSettings"))
  // Add in settings from the environment (with a prefix of APP)
  // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
  .add_source(config::Environment::with_prefix("API"))
  .build()
  .unwrap().try_deserialize::<ApiConfig>().unwrap()));
}