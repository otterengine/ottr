#![doc = include_str!("../README.md")]

/// `use ottr::prelude::*;` to import the most common types, traits and functions.
pub mod prelude {
    pub use ottr_main::prelude::*;

    #[cfg(feature = "ottr_window")]
    pub use ottr_window::prelude::*;
}
