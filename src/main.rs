use log::info;  // maybe we need this later {trace, debug, info, warn, error};

mod settings;
mod logger;
mod test;

const APP_NAME: &str = "vaibs-settings";
const APP_ORGANIZATION: &str = "vaibs";
const APP_QUALIFER: &str = "com";

fn main() {
    // Set up the logger with only support until debug
    logger::new(1).expect("Cannot start application an working logger systen");

    info!("API debug : {:?}", settings::api::data.read().unwrap().debug);
    info!("API addresss : {:?}", settings::api::data.read().unwrap().address);
    info!("API http port : {:?}", settings::api::data.read().unwrap().http_port);
    info!("API https port : {:?}", settings::api::data.read().unwrap().https_port);

    test::counting::counting();
    test::server::servers();

    *settings::api::data.write().unwrap() = settings::api::ApiConfig { debug: true, address: String::from("192.168.1.1"), http_port: 8080, https_port: 8443 };

    info!("API addresss : {:?}", settings::api::data.read().unwrap().address);
    info!("API http port : {:?}", settings::api::data.read().unwrap().http_port);
    info!("API https port : {:?}", settings::api::data.read().unwrap().https_port);

    test::api::api();

    info!("API addresss : {:?}", settings::api::data.read().unwrap().address);
    info!("API http port : {:?}", settings::api::data.read().unwrap().http_port);
    info!("API https port : {:?}", settings::api::data.read().unwrap().https_port);
}
