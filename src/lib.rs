//! # cbfr
//! This crate provide you with BFRDYN struct, a buffer
//! that store bytes data. Currently it's main purpose
//! is to provide a faster buffer to store and manipulate
//! small text. If you need a data structure that can store
//! larger text dynamically, use String instead.
//! BFRDYN is build on top of rust primitive array, you need
//! to provide a const generic to define buffer size, by default
//! the buffer size is 256. BFRDYN store data on the stack, so
//! becarefull that you didn't provide too large size that
//! will make your stack overflow.
//! # quick start
//! ```
//! use cbfr::cb::BFRDYN;
//!
//! let b1 = BFRDYN::<125>::from("some string");    // create a buffer with capacity = 125
//! let b2: BFRDYN = "another string".into();       // create a buffer with default capacity (256)
//! let mut b3: BFRDYN = BFRDYN::def();       // create a buffer with default capacity (256)
//! 
//! b3.append_str("more string");
//! assert_eq!(b1.to_string(), "some string");
//! assert_eq!(b2.to_string(), "another string");
//! assert_eq!(b3.to_string(), "more string");
//! ```
//! # [BFRDYN]


pub mod prelude;
pub mod cb;
pub mod helper;
pub mod errors;

/// Re-exports
pub use cb::BFRDYN;
pub use cb::DEFCAPACITY;

