#![cfg_attr(not(feature = "std"), no_std)]

pub mod deps {
    pub use prost;
}

mod more {
    include!(concat!(
        env!("OUT_DIR"),
        "/google/protobuf/google.protobuf.rs"
    ));
    #[cfg(feature = "serde-json")]
    include!(concat!(
        env!("OUT_DIR"),
        "/google/protobuf/google.protobuf.serde.rs"
    ));
}

pub mod wkt {
    #[cfg(not(feature = "serde-json"))]
    pub use prost_types::*;

    #[cfg(feature = "serde-json")]
    pub use pbjson_types::*;

    pub use super::more::Empty;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() -> Result<(), String> {
        let _ = wkt::Empty::default();
        Ok(())
    }
}
