use log::info;

use crate::settings;

pub fn counting() {
    info!("{:?}", settings::vaidio_counting::CONFIG.read().unwrap());
}

