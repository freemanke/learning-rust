use log::{debug, error, info, trace, warn};

#[test]
fn log() {
    env_logger::init();
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}