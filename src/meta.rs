mod size;

/// Metadata traits.
pub mod traits {
    pub use super::size::traits::Size;
}

/// Dynamic metadata types.
pub mod dynamic {
    pub use super::size::dynamic::Size;
}

pub use size::Size;
