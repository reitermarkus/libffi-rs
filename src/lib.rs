#![cfg_attr(feature = "unique", feature(unique))]

//! Rust bindings for [libffi](https://sourceware.org/libffi/).
//!
//! The C libffi library provides two main facilities: assembling calls
//! to functions dynamically, and creating closures that can be called
//! as ordinary C functions. In Rust, the latter means that we can turn
//! a Rust lambda (or any object implementing `Fn`/`FnMut`) into an
//! ordinary C function pointer that we can pass as a callback to C.
//!
//! The easiest way to use this library is via the
//! [`high`](high/index.html) layer module, but more flexibility (and
//! less checking) is provided by the [`middle`](middle/index.html) and
//! [`low`](low/index.html) layers.
//!
//! # Usage
//!
//! It’s [on crates.io](https://crates.io/crates/libffi), so it can be
//! used by adding `libffi` to the dependencies in your project’s
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! libffi = "0.1"
//! ```
//!
//! It is necessary to have C [libffi](https://sourceware.org/libffi/)
//! installed first. (We’ve tested with libffi
//! version 3.2.1.)
//!
//! # Organization
//!
//! This library is organized in four layers, each of which attempts to
//! provide more safety and a simpler interface than the next layer
//! down. From top to bottom:
//!
//!   - The [`high`](high/index.html) layer provides safe(?) and
//!     automatic marshalling of Rust closures into C function pointers.
//!   - The [`middle`](middle/index.html) layer provides memory-managed
//!     abstractions for assembling calls and closures, but is unsafe
//!     because it doesn’t check argument types.
//!   - The [`low`](low/index.html) layer makes no attempts at safety,
//!     but provides a more idiomatically “Rusty” API than the underlying
//!     C library.
//!   - The [`raw`](raw/index.html) layer is a direct mapping of the
//!     C libffi library into Rust, generated by
//!     [bindgen](https://crates.io/crates/bindgen).
//!
//! It should be possible to use any layer without dipping into lower
//! layers (and it will be considered a bug to the extent that it
//! isn’t).
//!
//! # Examples
//!
//! In this example, we convert a Rust lambda containing a free variable
//! into an ordinary C code pointer. The type of `fun` below is
//! `extern "C" fn(u64, u64) -> u64`.
//!
//! ```
//! use libffi::high::Closure2;
//!
//! let x = 5u64;
//! let f = |y: u64, z: u64| x + y + z;
//!
//! let closure = Closure2::new(&f);
//! let fun     = closure.code_ptr();
//!
//! assert_eq!(18, fun(6, 7));
//! ```

extern crate libc;

/// Unwrapped definitions imported from the C library (via bindgen).
///
/// This module is generated and undocumented, but you can see the [C
/// libffi documentation](libffi.txt).
pub mod raw;

pub mod high;
pub mod middle;
pub mod low;
