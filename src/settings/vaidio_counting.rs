use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VaidioServer {
    /// Vaidio server id number
    pub ainvr_id: u16,
    /// Vaidio server address => example http://192.168.1.1   
    pub address: String,
    /// Vaidio api prefix => /ainvr/api/ 
    pub prefix: String,
    /// Vaidio port number => 443 (0 = disable)
    pub port: u16,
    /// Vaidio timeout (0 = disable)
    pub timeout: u64,
    /// Vaidio authentication
    /// 0 = basic
    /// 1 = api key
    /// 2 = oauth 2.0
    pub auth: i64,
    /// Vaidio server auth username
    pub username: String,
    /// Vaidio server auth password    
    pub password: String,
    /// Vaidio server api key token
    pub api_key_token: String,
    /// Vaidio image prefix => 7.2 has /ainvr/samba/, 8.0 Edge device => doesn't need one for example   
    pub image_prefix: String,
}
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VaidioServers { pub server : Vec<VaidioServer> }

lazy_static! {
    pub static ref CONFIG: Arc<RwLock<VaidioServers>> = Arc::new(RwLock::new(Config::builder()
    .add_source(config::File::with_name("./config/VaidioServers"))
    .build()
    .unwrap().try_deserialize::<VaidioServers>().unwrap()));
  }