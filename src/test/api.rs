use crate::settings;

pub fn api() {
  *settings::api::data.write().unwrap() = settings::api::ApiConfig { debug: true, address: String::from("192.168.20.220"), http_port: 7080, https_port: 7443 };
}