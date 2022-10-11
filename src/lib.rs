// #![doc=include_str!("../README.md")]
//! Ultimate console progress bar for Rust.
//!
//! Inspired by [tqdm](https://github.com/tqdm/tqdm) & [rich.progress](https://rich.readthedocs.io/en/latest/progress.html)
//!
//! ## Quick Insight
//!
//! - [Bar](crate::Bar) can be used to create progress bars like tqdm.
//! - [RichProgress](crate::RichProgress) can be used to create progress bars like rich.progress.
//!
//! ## Examples
//!
//! - [One Page Usage](https://github.com/clitic/kdam#usage)
//! - [Project Examples](https://github.com/clitic/kdam/tree/main/examples)
//!
//! ```rust
//! use kdam::prelude::*;
//!
//! let mut pb = tqdm!(total = 100);
//!
//! for _ in 0..100 {
//!     pb.update(1);
//! }
//!
//! eprint!("\n");
//! ```
//! 
//! ## Optional Features
//!
//! The following are a list of [cargo features][cargo-features] that can be
//! enabled.
//!
//! - **template**: Enables templating capabilities for [Bar](crate::Bar).
//! - **gradient**: Enables gradient colours for progress bars and printing text.

mod progress;
mod styles;
mod thread;

pub mod prelude;
pub mod term;

pub use progress::*;
pub use styles::{Animation, Spinner};
pub use thread::RowManager;

pub use styles::format;
pub use thread::monitor;
