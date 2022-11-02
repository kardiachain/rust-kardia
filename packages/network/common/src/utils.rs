use futures::{Stream, stream::unfold, FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;

/// Creates a stream that returns a new value every `duration`.
pub fn interval(duration: Duration) -> impl Stream<Item = ()> + Unpin {
	unfold((), move |_| Delay::new(duration).map(|_| Some(((), ())))).map(drop)
}
