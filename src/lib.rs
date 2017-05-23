#![deny(missing_docs)]

//! This module provides a way to wrap Futures
//! in another Future that logs internal calls
//! to poll. This allows a better understanding
//! for when polls happen for both teaching
//! and debugging.
extern crate futures;

#[cfg(not(feature="silence"))]
#[macro_use]
extern crate log;

use futures::{Future, Poll};
use std::fmt::Debug;

/// The LoggedFuture struct wraps another Future and
/// will log all poll calls.
#[derive(Debug)]
pub struct LoggedFuture<T, E, F: Future<Item = T, Error = E>> {
    future: F,
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

/// LoggingExt introduces the logging capabilities
/// to any Future, as long as all its Item and Error
/// can be printed.
pub trait LoggingExt<T, E>
    where T: Debug,
          E: Debug,
          Self: Future<Item = T, Error = E> + Sized
{
    /// inspect() sets up the logging. The `label` will
    /// be used to identify the Future in the log messages
    /// used.
    ///
    /// This method returns `Self` instead of a `LoggedFuture`
    /// when the `silence` feature is activated.
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
