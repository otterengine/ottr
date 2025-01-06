//! # ottr_main
//!
//! Provides utilities for defining a [`MainLoop`] for an Otter application.
//!

use core::future::Future;

#[cfg(feature = "std")]
use std::process::{ExitCode, Termination};

/// `use ottr_main::prelude::*;` to import the most common types, traits and functions.
pub mod prelude {
    pub use crate::{MainLoop, MainLoopExit};
}

/// The value [`MainLoop::run`] returns to indicate success or failure.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct MainLoopExit(pub u8);

impl<T: Into<u8>> From<T> for MainLoopExit {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[cfg(feature = "std")]
impl Termination for MainLoopExit {
    fn report(self) -> ExitCode {
        ExitCode::from(self.0)
    }
}

/// An interface used to run an Otter application.
pub trait MainLoop {
    /// Run the main loop.
    fn run(self) -> impl Future<Output = MainLoopExit>;
}
