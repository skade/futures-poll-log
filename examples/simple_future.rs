extern crate futures;
extern crate futures_poll_log;
extern crate log;

use futures_poll_log::LoggingExt;
use futures::Future;

use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};

fn main() {
    log::set_logger(|max_log_level| {
                        max_log_level.set(LogLevelFilter::Trace);
                        Box::new(SimpleLogger)
                    })
            .unwrap();

    let _: Result<i32, _> = futures::future::ok(3)
        .inspect("immeditate future")
        .map(|i| i * 2)
        .inspect("mapped future")
        .and_then(|_| Err("ooops".to_string()))
        .inspect("failing future")
        .wait();
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
