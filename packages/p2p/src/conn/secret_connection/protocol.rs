

/// Protocol version (based on the Tendermint version)
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Version {
    V1_0_0,
}