//! Compile-time duration parsing.
//!
//! ```rust
//! use core::time::Duration;
//! use duration_macro::duration;
//!
//! assert_eq!(duration!(2 d 1 m), Duration::from_secs(3600 * 24 * 2 + 60 * 1));
//! assert_eq!(duration!(1 m 2 d), Duration::from_secs(3600 * 24 * 2 + 60 * 1));
//! assert_eq!(duration!(100 ns), Duration::from_nanos(100));
//! assert_eq!(duration!({100 * 2} ns), Duration::from_nanos(200));
//! ```
//!
//! For more details, please see the [duration!] docs.

#![no_std]
#![deny(missing_docs)]

/// Compile-time duration parsing macro.
///
/// The macro accepts duration in a form of `{block} {unit} [{block} {unit}
/// ...]`, where `{block}` is a ["token
/// tree"](https://doc.rust-lang.org/reference/macros.html#macro-invocation),
/// and `{unit}` is one of the following literals:
/// * `d` or `day` or `days` for days,
/// * `s` for seconds,
/// * `ms` for milliseconds,
/// * `us` for microseconds,
/// * `ns` for nanoseconds.
///
/// ```rust
/// use core::time::Duration;
/// use duration_macro::duration;
///
/// let reference_duration = Duration::new(86400 + 3600 * 2 + 3 * 60 + 4, 5 * 1_000_000 + 6 * 1_000 + 7);
/// assert_eq!(
///     duration!(1 d 2 h 3 m 4 s 5 ms 6 us 7 ns),
///     reference_duration
/// );
/// // The order doesn't matter!
/// assert_eq!(
///     duration!(6 us 2 h 3 m 4 s 1 d 5 ms 7 ns),
///     reference_duration
/// );
/// // Expressions can be passed using `{..}` blocks:
/// assert_eq!(
///     duration!({3 * 2} ns {7 / 2} d),
///     Duration::from_nanos(6) + Duration::from_secs(3 * 86400)
/// );
/// ```
#[macro_export]
macro_rules! duration {
    ($something:tt $name:ident $($something2:tt $name2:ident)+) => {
        $crate::duration!($something $name) $( + $crate::duration!($something2 $name2) )+
    };
    ($nanoseconds:tt ns) => {
        ::core::time::Duration::from_nanos($nanoseconds)
    };
    ($milliseconds:tt ms) => {
        ::core::time::Duration::from_millis($milliseconds)
    };
    ($microseconds:tt us) => {
        ::core::time::Duration::from_micros($microseconds)
    };
    ($seconds:tt s) => {
        ::core::time::Duration::from_secs($seconds)
    };
    ($minutes:tt m) => {
        ::core::time::Duration::from_secs($minutes * 60)
    };
    ($hours:tt h) => {
        ::core::time::Duration::from_secs($hours * 60 * 60)
    };
    ($days:tt d) => {
        ::core::time::Duration::from_secs($days * 60 * 60 * 24)
    };
    ($days:tt day) => {
        $crate::duration!($days d)
    };
    ($days:tt days) => {
        $crate::duration!($days d)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(
            duration!(1 d 2 h 3 m 4 s 5 ms 6 us 7 ns),
            Duration::new(86400 + 3600 * 2 + 3 * 60 + 4, 5 * 1_000_000 + 6 * 1_000 + 7)
        );
    }

    #[test]
    fn days() {
        assert_eq!(duration!(8 d), Duration::from_secs(8 * 86400));
        assert_eq!(duration!(8 day), Duration::from_secs(8 * 86400));
        assert_eq!(duration!(8 days), Duration::from_secs(8 * 86400));
    }
}
