
pub trait Service {

}

pub struct BaseService {
    name: String,
    started: u32,
    stopped: u32,

    /// change into rust channel
    quit: bool,
}

impl BaseService {
    pub fn set_logger() {}

    pub fn start() {}

    pub fn on_start() {}

    pub fn stop() {}

    pub fn on_stop() {}

    pub fn reset() {}

    pub fn on_reset() {}

    pub fn is_running() -> bool {
        false
    }

    pub fn wait() {}

    pub fn string() -> String {
        "".to_str()
    }

    pub fn quit() {}
}