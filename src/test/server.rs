use log::info;

use crate::settings;

pub fn servers() {
    info!("{:?}", settings::vaidio_server::CONFIG.read().unwrap());
}