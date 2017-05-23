# Futures Poll Log

This crate adds a "inspect" method to all futures, allowing you to tag futures with a name and see all poll calls to them logged.

## Usage

Setup a logger through the `log` crate. Then use the extension trait:

```rust
extern crate futures_poll_log;
use futures_poll_log::LoggingExt;
```

```rust
let _: Result<i32, _> =futures::future::ok(3)
        .inspect("immeditate future")
        .map(|i| {
            i*2
        })
        .inspect("mapped future")
        .and_then(|_| {
            Err("ooops".to_string())
        })
        .inspect("failing future")
        .wait();
```

This will log:

```rust
DEBUG - Polling future `failing future'
DEBUG - Polling future `mapped future'
DEBUG - Polling future `immeditate future'
DEBUG - Future `immeditate future' polled: Ok(Ready(3))
DEBUG - Future `mapped future' polled: Ok(Ready(6))
DEBUG - Future `failing future' polled: Err("ooops")
```

Note that it logs the Async state.

### Log target

The log target is `futures_log`.

### Silence

Building the crate with the feature "silence" makes the effect completely vanish, _including_ the intermediate futures. The library also stops binding to `log` lib.

This allows you to keep the tagging around for future debugging sessions.

## License

MIT