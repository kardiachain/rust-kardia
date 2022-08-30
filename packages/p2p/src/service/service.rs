use std::error::Error;

// Service defines a service that can be started, stopped, and reset.
pub trait Service {
    // Start the service.
	// If it's already started or stopped, will return an error.
	// If OnStart() returns an error, it's returned by Start()
    fn start() -> Result<(), Box<dyn Error>>;

    fn on_start() -> Result<(), Box<dyn Error>>;

    // Stop the service.
	// If it's already stopped, will return an error.
	// OnStop must never error.
    fn stop() -> Result<(), Box<dyn Error>>;
    fn on_stop();

    // Reset the service.
	// Panics by default - must be overwritten to enable reset.
    fn reset() -> Result<(), Box<dyn Error>>;
    fn on_reset() -> Result<(), Box<dyn Error>>;

    // Return true if the service is running
    fn is_running() -> bool;

    // Quit returns a channel, which is closed once service is stopped.
    // TODO: will define later.
    fn quit();

    // String representation of the service
    fn string() -> String;

    // SetLogger sets a logger.
    fn set_logger();
}

pub struct BaseService {
    name : String,
    started: u32,
    stopped: u32,
    quit: String,   // re-define channel in go to rust

    // The "subclass" of BaseService
    // implement: Box::new<dyn Service>,
}

pub fn new_base_service() -> &'static BaseService {
    todo!()
}

impl Service for BaseService {
    fn start() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn on_start() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn stop() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn on_stop() {
        todo!()
    }

    fn reset() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn on_reset() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn is_running() -> bool {
        todo!()
    }

    fn quit() {
        todo!()
    }

    fn string() -> String {
        todo!()
    }

    fn set_logger() {
        todo!()
    }
}