pub type ChannelId = u8;
pub type Message = Vec<u8>;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// Txs that will be applied by state @ block.Height+1.
    /// NOTE: not all txs here are valid.  We're just agreeing on the order first.
    /// This means that block.AppHash does not include these txs.
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}