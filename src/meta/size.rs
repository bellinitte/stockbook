pub mod traits {
    /// A generalized size of a [`Stamp`](crate::Stamp).
    pub trait Size {
        /// Size of the stamp in pixels &mdash; width and height, or columns and rows.
        fn size(&self) -> [usize; 2];
    }
}

pub mod dynamic {
    use super::traits;

    /// Metadata type that holds the information about the size of a
    /// [`Stamp`](crate::Stamp) at runtime.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Size {
        pub(super) width: usize,
        pub(super) height: usize,
    }

    impl traits::Size for Size {
        fn size(&self) -> [usize; 2] {
            [self.width, self.height]
        }
    }
}

/// Metadata type that holds the information about the size of a
/// [`Stamp`](crate::Stamp) at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Size<const WIDTH: usize, const HEIGHT: usize>;

impl<const WIDTH: usize, const HEIGHT: usize> traits::Size for Size<WIDTH, HEIGHT> {
    fn size(&self) -> [usize; 2] {
        [WIDTH, HEIGHT]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Size<WIDTH, HEIGHT> {
    pub(crate) const fn downgrade(self) -> dynamic::Size {
        dynamic::Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}
