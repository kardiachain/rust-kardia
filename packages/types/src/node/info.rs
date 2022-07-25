use crate::node;

pub struct Info {
    /// Node Id
    pub id: node::Id,

    /// Listen address
    pub listen_addr: ListenAddress,

    /// Moniker
    /// TODO: change into Moniker struct?
    pub moniker: String,

}

/// Listen address information
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListenAddress(String);