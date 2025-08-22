#![cfg_attr(not(feature = "std"), no_std)]

pub mod wkt {
    #[cfg(not(feature = "serde-json"))]
    pub use prost_types::*;

    #[cfg(feature = "serde-json")]
    pub use pbjson_types::*;
}
