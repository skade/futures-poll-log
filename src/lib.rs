//! Add `inspect` to all futures for easy debugging.
//!
//! This module provides a way to wrap `Future`s in another `Future` that logs
//! internal calls to poll. This allows a better understanding for when polls
//! happen for both teaching and debugging.
//!
//! This uses the `log` crate internally, so you need to have a logger set up
//! to see any output (e.g., by using the `env_logger` crate).
//!
//! To get access to the `inspect` methid, import the crate and load the
//! extension trait into scope with `use futures_poll_log::LoggingExt`.
//!
//! # Examples
//!
//! ```rust
//! extern crate futures;
//! extern crate futures_poll_log;
//!
//! use futures::{Future, future};
//! use futures_poll_log::LoggingExt;
//!
//! # fn main() {
//! let result: Result<i32, _> =
//!     future::ok(3)
//!     .inspect("immeditate future")
//!     .map(|i| i * 2)
//!     .inspect("mapped future")
//!     .and_then(|_| Err("ooops".to_string()))
//!     .inspect("failing future")
//!     .wait();
//! # }
//! ```
//!
//! This will log:
//!
//! ```plain
//! DEBUG - Polling future `failing future'
//! DEBUG - Polling future `mapped future'
//! DEBUG - Polling future `immeditate future'
//! DEBUG - Future `immeditate future' polled: Ok(Ready(3))
//! DEBUG - Future `mapped future' polled: Ok(Ready(6))
//! DEBUG - Future `failing future' polled: Err("ooops")
//! ```
//!
//! Note that it logs the Async state.
//!
//! # Notes on logging
//!
//! The log target is `futures_log`.
//!
//! Building the crate with the feature "silence" makes the effect completely
//! vanish, _including_ the intermediate futures. The library also stops binding
//! to `log` lib.
//!
//! This allows you to keep the tagging around for future debugging sessions.

#![deny(missing_docs)]

extern crate futures;

#[cfg(not(feature="silence"))]
#[macro_use]
extern crate log;
use  futures::Async;
use futures::{Future, Poll};
use std::fmt::Debug;

/// The LoggedFuture struct wraps another Future and
/// will log all poll calls with content of the poll.
#[derive(Debug)]
pub struct LoggedFuture<T, E, F: Future<Item = T, Error = E>> {
    future: F,
    label: String
}


/// The LoggedFuture struct wraps another Future and
/// will log all poll calls with poll result.
#[derive(Debug)]
pub struct LoggedFutureSimple<T, E, F: Future<Item = T, Error = E>> {
    future: F,
    label: String
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

#[cfg(not(feature="silence"))]
impl<T, E, F> Future for LoggedFutureSimple<T, E, F>
    where E: Debug,
          F: Future<Item = T, Error = E>
{
    type Item = F::Item;
    type Error = F::Error;

    #[inline]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!(target: "futures_log", "Polling future `{}'", self.label);
        let poll = self.future.poll();
            match &poll{
                &Ok(Async::Ready(_))=>{ debug!(target: "futures_log", "Future `{}' polled and is ready", self.label);},
                &Ok(Async::NotReady)=>{ debug!(target: "futures_log", "Future `{}' polled and is not ready", self.label);},
                &Err(ref e)=>{ debug!(target: "futures_log", "Future `{}' polled and  errored {:?}", self.label,e);},
            }
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

#[cfg(feature="silence")]
impl<T, E, F> Future for LoggedFutureSimple<T, E, F>
    where E: Debug,
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

/// LoggingExtSimple introduces the logging capabilities
/// to any Future, as long as its Error
/// can be printed.
pub trait LoggingExtSimple<T, E>
    where Self: Future<Item = T, Error = E> + Sized
{
    /// inspect_simple() sets up the logging. The `label` will
    /// be used to identify the Future in the log messages
    /// used.
    ///
    /// This method returns `Self` instead of a `LoggedFutureSimple`
    /// when the `silence` feature is activated.
    #[cfg(not(feature="silence"))]
    fn inspect_simple(self, label: &str) -> LoggedFutureSimple<T, E, Self>;
    #[cfg(feature="silence")]
    fn inspect_simple(self, label: &str) -> Self;
}

impl<T, E, F> LoggingExtSimple<T, E> for F
    where  Self: Future<Item = T, Error = E>
{
    #[cfg(not(feature="silence"))]
    fn inspect_simple(self, label: &str) -> LoggedFutureSimple<T, E, Self> {
        LoggedFutureSimple {
            future: self,
            label: label.to_owned()
        }
    }
    #[cfg(feature="silence")]
    fn inspect_simple(self, _: &str) -> Self {
        self
    }
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
            label: label.to_owned()
        }
    }
    #[cfg(feature="silence")]
    fn inspect(self, _: &str) -> Self {
        self
    }
}
