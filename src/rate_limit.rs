use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::stream::Stream;
use tokio::time::{interval, Interval, MissedTickBehavior};

fn assert_stream<T, S>(stream: S) -> S
where
    S: Stream<Item = T>,
{
    stream
}

fn limiter(per_sec: u64) -> Interval {
    let delay_ms = ((1.0 / per_sec as f64) * 1_000.0) as u64;
    let mut limiter = interval(Duration::from_millis(delay_ms));
    limiter.set_missed_tick_behavior(MissedTickBehavior::Delay);
    limiter
}

pub struct RateLimitIter<I: Unpin> {
    iter: I,
    timer: Interval,
}

pub fn rate_limit<I>(i: I, per_sec: u64) -> RateLimitIter<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Unpin,
{
    assert_stream::<I::Item, _>(RateLimitIter {
        iter: i.into_iter(),
        timer: limiter(per_sec),
    })
}

impl<I: Unpin> RateLimitIter<I> {
    pub fn set_missed_tick_behavior(&mut self, opt: MissedTickBehavior) {
        self.timer.set_missed_tick_behavior(opt);
    }
}

impl<I> Stream for RateLimitIter<I>
where
    I: Iterator + Unpin,
{
    type Item = I::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<I::Item>> {
        match self.timer.poll_tick(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(self.iter.next()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
