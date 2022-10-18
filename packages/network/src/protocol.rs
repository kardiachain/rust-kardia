use std::sync::Arc;

/// The protocol name transmitted on the wire.
#[derive(Debug, Clone)]
pub enum ProtocolName {
	/// The protocol name as a static string.
	Static(&'static str),
	/// The protocol name as a dynamically allocated string.
	OnHeap(Arc<str>),
}