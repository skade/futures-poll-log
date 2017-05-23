extern crate futures;

#[cfg(not(feature="silence"))]
#[macro_use]
extern crate log;

use futures::{Future, Poll};
use std::fmt::Debug;

#[derive(Debug)]
pub struct LoggedFuture<T, E, F: Future<Item = T, Error = E>> {
    future: F,
    #[cfg(not(silence))]
    label: String,
}

#[cfg(not(feature="silence"))]
impl<T, E, F> Future for LoggedFuture<T, E, F>
    where T: Debug,
          E: Debug,
          F: Future<Item = T, Error = E>
{
    type Item = F::Item;
    type Error = F::Error;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!(target: "futures_log", "Polling future `{}'", self.label);
        let poll = self.future.poll();
        debug!(target: "futures_log", "Future `{}' polled: {:?}", self.label, poll);
        poll
    }
}

#[cfg(feature="silence")]
impl<T, E, F> Future for LoggedFuture<T, E, F>
    where T: Debug,
          E: Debug,
          F: Future<Item = T, Error = E>
{
    type Item = F::Item;
    type Error = F::Error;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

pub trait LoggingExt<T, E>
    where T: Debug,
          E: Debug,
          Self: Future<Item = T, Error = E> + Sized
{
    #[cfg(not(feature="silence"))]
    fn inspect(self, label: &str) -> LoggedFuture<T, E, Self>;
    #[cfg(feature="silence")]
    fn inspect(self, label: &str) -> Self;
}

impl<T, E, F> LoggingExt<T, E> for F
    where T: Debug,
          E: Debug,
          Self: Future<Item = T, Error = E>
{
    #[cfg(not(feature="silence"))]
    fn inspect(self, label: &str) -> LoggedFuture<T, E, Self> {
        LoggedFuture {
            future: self,
            label: label.to_owned(),
        }
    }
    #[cfg(feature="silence")]
    fn inspect(self, _: &str) -> Self {
        self
    }
}
