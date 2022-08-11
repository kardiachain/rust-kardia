pub struct Part {
    pub index: u32,
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    pub proof: ::core::option::Option<super::crypto::Proof>,
}