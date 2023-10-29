# Low Priority Thread
![CI](https://github.com/hinto-janai/lpt/actions/workflows/ci.yml/badge.svg) [![crates.io](https://img.shields.io/crates/v/lpt.svg)](https://crates.io/crates/lpt) [![docs.rs](https://docs.rs/lpt/badge.svg)](https://docs.rs/lpt)

This is a 1-function crate that sets the calling thread's priority to the lowest platform-specific value possible.

```rust
// Set the current thread to the lowest priority.
//
// This function returns () and will never fail.
lpt::lpt();
```

## Windows
Uses [`SetThreadPriority()`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority) with `THREAD_PRIORITY_IDLE` (`-15`).

## Unix
Uses [`libc::nice()`](https://man7.org/linux/man-pages/man2/nice.2.html) with the max nice level.

- On `macOS` and `*BSD`: `+20`
- On `Linux`: `+19`
