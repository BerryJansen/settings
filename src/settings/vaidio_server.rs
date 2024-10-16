use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub struct CountingSettingsSection {
    /// Descriptions of section
    pub desc: String,
    /// Line ID that should be counted as IN-OUT
    pub line_id: Vec<u16>,
    /// Line ID that should be counted as OUT-IN
    pub line_id_rev: Vec<u16>,
}

#[derive(Default, Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub struct CountingSettingsServer {
    pub ainvr_id: u16,
    pub line_id: Vec<u16>,
    /// Object counting type label like "car,truck,bus,person,bicycle,motorbike"
    /// separated bij comma
    pub types: String,
}

#[derive(Default, Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub struct CountingSettings {
    /// Configuration of sections of server
    pub sections: Vec<CountingSettingsSection>,
    /// Line ID that are active per server
    pub servers: Vec<CountingSettingsServer>,
}

lazy_static! {
    pub static ref CONFIG: Arc<RwLock<CountingSettings>> = Arc::new(RwLock::new(Config::builder()
    .add_source(config::File::with_name("./config/VaidioCounting"))
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    .add_source(config::Environment::with_prefix("API"))
    .build()
    .unwrap().try_deserialize::<CountingSettings>().unwrap()));
  }