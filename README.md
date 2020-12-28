# duration-macro

[![crates.io](https://img.shields.io/crates/v/duration-macro.svg)](https://crates.io/crates/duration-macro)
[![docs.rs](https://docs.rs/duration-macro/badge.svg)](https://docs.rs/duration-macro)

Compile-time duration parsing.

```rust
use core::time::Duration;
use duration_macro::duration;

assert_eq!(duration!(2 d 1 m), Duration::from_secs(3600 * 24 * 2 + 60 * 1));
assert_eq!(duration!(1 m 2 d), Duration::from_secs(3600 * 24 * 2 + 60 * 1));
assert_eq!(duration!(100 ns), Duration::from_nanos(100));
assert_eq!(duration!({100 * 2} ns), Duration::from_nanos(200));
```

For more details, please see the [duration!](https://docs.rs/duration-macro/*/duration_macro/macro.duration.html) docs.

License: MIT/Apache-2.0
