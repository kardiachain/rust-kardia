

/// Protocol version (based on the Tendermint version)
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Version {
    // Default protocol version for Kardia
    V1_0_0,
}

impl Version {
    /// Does this version of Secret Connection use a transcript hash
    #[must_use]
    pub fn has_transcript(self) -> bool {
        self != Self::V1_0_0
    }
}