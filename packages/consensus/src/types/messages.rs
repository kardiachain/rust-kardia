use std::error::Error;

/**
    Message is a message that can be sent and received on the `ConsensusReactor`
 */
pub trait Message {
    fn validate_basic() -> Result<(), Box<dyn Error>>;
}
