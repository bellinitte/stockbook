//! Stockbook embeds 1-bit raster images in your code at compile time.
//!
//! Designed primarily for `#![no_std]` usage, in embedded or other
//! program-memory-constrained environments.
//!
//! The main functionality of Stockbook is the [`stamp!`] macro, which lets you
//! include data similarly to how [`include_bytes!`] does, but from an image,
//! specifically a 1-bit black and white image. The macro returns a [`Stamp`]
//! type, which just holds a static reference to the pixel data &mdash; the size of
//! the image is encoded statically in the type. The pixel data is represented
//! internally as an array of bytes, in which individual bits correspond to
//! individual pixels.
//!
//! ## Example
//!
//! File `assets/star.png` (scaled x8 for preview, originally 12x12 px):
//!
//! ![Star](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGAAAABgCAYAAADimHc4AAAAAXNSR0IArs4c6QAAAT5JREFUeJzt3FEKwyAUAEEtvf+V0yP4YWUSsnMCy/Igxtg5xrjGjV3X3vLmnH9ayRkfvYC3KwBWAKwAWAGwAmAFwAqAFQArAFYArABYAbACYAXA5sDnAbvv+3fp84ImACsAVgCsAFgBsAJgBcAKgBUAKwBWAKwAWAGwAmAFwOalX8i/XBOAFQArAFYArABYAbACYMvvgtom7Fl9d9QEYAXACoAVACsAVgCsANj2/YC37xN27xc0AVgBsAJgBcAKgBUAKwB2/J7w0/cJp+8RNwFYAbACYAXACoAVACsA1nnAps4DHq4AWAGwAmAFwAqAFQD76gWs7D5n332f0gRgBcAKgBUAKwBWAKwAGL8nrP+/X/++JgArAFYArABYAbACYAXAjp8H6Of8ldX6Tu8TmgCsAFgBsAJgBcAKgBUA+wFZhym1RhU7SwAAAABJRU5ErkJggg==)
//!
//! File `src/lib.rs`:
//!
//! ```rust
//! use stockbook::{stamp, Color, Size, Stamp};
//!
//! # const STAR_DATA: &[u8] = &[
//! #     0b00000110, 0b00000000, 0b01100000, 0b00001111, 0b00000000, 0b11110000,
//! #     0b11111111, 0b11110111, 0b11111110, 0b00111111, 0b11000001, 0b11111000,
//! #     0b00111111, 0b11000011, 0b10011100, 0b01110000, 0b11100110, 0b00000110,
//! # ];
//! #
//! # const EXPECTED_PIXELS: &[(usize, usize)] = &[
//! #     (5, 0), (6, 0), (5, 1), (6, 1), (4, 2), (5, 2), (6, 2), (7, 2),
//! #     (4, 3), (5, 3), (6, 3), (7, 3), (0, 4), (1, 4), (2, 4), (3, 4),
//! #     (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4), (11, 4),
//! #     (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (6, 5), (7, 5), (8, 5),
//! #     (9, 5), (10, 5), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6),
//! #     (8, 6), (9, 6), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7),
//! #     (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8),
//! #     (2, 9), (3, 9), (4, 9), (7, 9), (8, 9), (9, 9), (1, 10), (2, 10),
//! #     (3, 10), (8, 10), (9, 10), (10, 10), (1, 11), (2, 11), (9, 11),
//! #     (10, 11),
//! # ];
//! #
//! # static mut ACTUAL_PIXELS: Vec<(usize, usize)> = Vec::new();
//! #
//! # macro_rules! stamp {
//! #     ($path:literal) => { Stamp::<Size<12, 12>>::from_raw(&STAR_DATA) };
//! # }
//! static STAR_SPRITE: Stamp<Size<12, 12>> = stamp!("assets/star.png");
//!
//! pub fn draw_star() {
//!     for (x, y, color) in STAR_SPRITE.pixels() {
//!         match color {
//!             Color::Black => {}, // Treat as transparent
//!             Color::White => draw_pixel_at(x, y),
//!         }
//!     }
//! }
//!
//! fn draw_pixel_at(x: usize, y: usize) {
//!     /* ... */
//!     # unsafe { ACTUAL_PIXELS.push((x, y)); }
//! }
//! # draw_star();
//! # assert_eq!(unsafe { ACTUAL_PIXELS.as_slice() }, EXPECTED_PIXELS);
//! ```
//!
//! ## Supported formats
//!
//! Stockbook uses the [image](https://docs.rs/image) crate under the hood. See its
//! own [list of supported formats](https://docs.rs/image/latest/image/codecs/index.html#supported-formats)
//! for more details.
//!
//! ## Unstable features
//!
//! Although this library works on `stable`, any changes to images referenced by the
//! [`stamp!`] macro might not be detected because of caching. Therefore, until
//! [`track_path` API](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html)
//! ([Tracking Issue](https://github.com/rust-lang/rust/issues/99515)) stabilizes,
//! it is recommended to use the `nightly` toolchain, however functionality behind
//! this feature is unstable and may change or stop compiling at any time.

#![no_std]
#![warn(missing_docs)]

mod iter;
mod meta;

use iter::*;

pub use meta::*;
pub use stockbook_stamp_macro::stamp;

/// Rectangular, 1-bit, raster image.
///
/// A stamp is defined by its width, height, and the color of its pixels, of which
/// there are two: [`Black`](Color::Black) and [`White`](Color::White). Coordinate
/// _(0, 0)_ is the top-left corner of the stamp.
///
/// Stamp's pixel colors are represented internally as an array of bytes, in which
/// individual bits correspond to individual pixels. The last byte must be padded
/// and the rest of the slice is completely ignored.
#[derive(Debug, Clone, Copy)]
pub struct Stamp<S: traits::Size = dynamic::Size> {
    size: S,
    data: &'static [u8],
}

impl<const WIDTH: usize, const HEIGHT: usize> Stamp<Size<WIDTH, HEIGHT>> {
    /// Constructs a stamp and validates the length of `data`.
    ///
    /// This is a quasi-internal API &mdash; the intended way of constructing [`Stamp`]s
    /// is via the [`stamp!`] macro.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `data` does not match the number of
    /// pixels, which is assumed to be `width * height`.
    ///
    /// For example, here the dimensions of the stamp are 3x3, so 9 pixels in total, and
    /// so `data` must contain at least 9 bits (2 bytes rounding up), which it does:
    ///
    /// ```rust
    /// # use stockbook::{Size, Stamp};
    /// let stamp = Stamp::<Size<3, 3>>::from_raw(&[0b11111111, 0b1_0000000]);
    /// ```
    ///
    /// Here, only 8 bits are provided, so the function panics:
    ///
    /// ```rust,should_panic
    /// # use stockbook::{Size, Stamp};
    /// let stamp = Stamp::<Size<3, 3>>::from_raw(&[0b11111111]);
    /// ```
    ///
    /// Similarly here, but in a const context, the program fails to compile:
    ///
    /// ```rust,compile_fail
    /// # use stockbook::{Size, Stamp};
    /// static STAMP: Stamp<Size<3, 3>> = Stamp::<Size<3, 3>>::from_raw(&[0b11111111]);
    /// ```
    pub const fn from_raw(data: &'static [u8]) -> Self {
        if Self::bytes_count(WIDTH * HEIGHT) > data.len() {
            panic!("length of `data` doesn't match the number of pixels");
        }

        // SAFETY: we just checked that the length of `data` matches the number of pixels
        unsafe { Self::from_raw_unchecked(data) }
    }

    /// Constructs a stamp without any checks on the length of `data`.
    ///
    /// For a safe alternative see [`from_raw`](Stamp::from_raw) or the [`stamp!`]
    /// macro.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the length of `data` matches the number of pixels.
    pub const unsafe fn from_raw_unchecked(data: &'static [u8]) -> Self {
        Self { size: Size, data }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Stamp<Size<WIDTH, HEIGHT>> {
    /// Erases a type-level information about the stamp's size, converting a
    /// `Stamp<Size<WIDTH, HEIGHT>>` to a `Stamp<dynamic::Size>`. Useful if you don't
    /// care about the size of the stamp at compile time, or if you want to convert
    /// multiple different stamps into a single type. Do note, however, that using a
    /// dynamic size has a runtime cost &mdash; the width and height have to be kept
    /// _somewhere_.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<::stockbook::Size<3, 2>>::from_raw(&[0b000_000_00]) };
    /// # }
    /// static IMAGE: Stamp = stamp!("image.png").downgrade();
    /// ```
    pub const fn downgrade(self) -> Stamp {
        Stamp {
            size: self.size.downgrade(),
            data: self.data,
        }
    }
}

impl<S: traits::Size> Stamp<S> {
    /// Size of the stamp in pixels &mdash; width and height, or columns and rows.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 2>>::from_raw(&[0b000_000_00]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 2>> = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.size(), [3, 2]);
    /// ```
    #[inline]
    pub fn size(&self) -> [usize; 2] {
        self.size.size()
    }

    /// Width of the stamp in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 2>>::from_raw(&[0b000_000_00]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 2>> = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.width(), 3);
    /// ```
    #[inline]
    pub fn width(&self) -> usize {
        self.size()[0]
    }

    /// Height of the stamp in pixels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #    ($path:literal) => { Stamp::<Size<3, 2>>::from_raw(&[0b000_000_00]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 2>> = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.height(), 2);
    /// ```
    #[inline]
    pub fn height(&self) -> usize {
        self.size()[1]
    }

    /// Number of pixels in the stamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 2>>::from_raw(&[0b000_000_00]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 2>> = stamp!("image_3x2.png");
    ///
    /// assert_eq!(IMAGE.pixel_count(), 6);
    /// ```
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.width() * self.height()
    }

    /// Checks if a given coordinate is within the bounds of the image.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<5, 4>>::from_raw(&[0b00000000, 0b00000000, 0b0000_0000]) };
    /// # }
    /// static IMAGE: Stamp<Size<5, 4>> = stamp!("image_5x4.png");
    ///
    /// assert!(IMAGE.is_within_bounds(0, 0));
    /// assert!(IMAGE.is_within_bounds(4, 3));
    /// assert!(!IMAGE.is_within_bounds(5, 3));
    /// assert!(!IMAGE.is_within_bounds(4, 4));
    /// ```
    pub fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    /// Returns an iterator over all pixels of a [`Stamp`]. The iteration order is
    /// _x_ from 0 to _width_, then _y_ from 0 to _height_. A pixel is a
    /// _(x, y, color)_ tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 3>>::from_raw(&[0b101_010_10, 0b1_0000000]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 3>> = stamp!("checkerboard_3x3.png");
    ///
    /// let mut pixels = IMAGE.pixels();
    ///
    /// assert_eq!(pixels.next(), Some((0, 0, Color::White)));
    /// assert_eq!(pixels.next(), Some((1, 0, Color::Black)));
    /// assert_eq!(pixels.next(), Some((2, 0, Color::White)));
    /// assert_eq!(pixels.next(), Some((0, 1, Color::Black)));
    /// # for _ in 0..4 {
    /// #     pixels.next();
    /// # }
    /// /* ... */
    /// assert_eq!(pixels.next(), Some((2, 2, Color::White)));
    /// assert_eq!(pixels.next(), None);
    /// ```
    pub fn pixels(&self) -> Pixels<'_, S> {
        Pixels::new(self)
    }

    /// Yields the color of the stamp at the provided coordinate. Panicking version of
    /// [`get_color_checked`](Stamp::get_color_checked).
    ///
    /// # Panics
    ///
    /// This method panics if the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 3>>::from_raw(&[0b101_010_10, 0b1_0000000]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 3>> = stamp!("checkerboard_3x3.png");
    ///
    /// assert_eq!(IMAGE.get_color(0, 0), Color::White);
    /// assert_eq!(IMAGE.get_color(1, 0), Color::Black);
    /// assert_eq!(IMAGE.get_color(0, 1), Color::Black);
    /// ```
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.get_color_checked(x, y).expect("")
    }

    /// Yields the color of the stamp at the provided coordinate. Returns [`None`] if
    /// the coordinate is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 3>>::from_raw(&[0b101_010_10, 0b1_0000000]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 3>> = stamp!("checkerboard_3x3.png");
    ///
    /// assert_eq!(IMAGE.get_color_checked(0, 0), Some(Color::White));
    /// assert_eq!(IMAGE.get_color_checked(1, 0), Some(Color::Black));
    /// assert_eq!(IMAGE.get_color_checked(3, 0), None);
    /// assert_eq!(IMAGE.get_color_checked(0, 3), None);
    /// ```
    pub fn get_color_checked(&self, x: usize, y: usize) -> Option<Color> {
        if !self.is_within_bounds(x, y) {
            return None;
        }

        // SAFETY: we just checked the coordinates are within the bounds of the stamp
        let color = unsafe { self.get_color_unchecked(x, y) };
        Some(color)
    }

    /// Yields the color of the stamp at the provided coordinate, without doing bounds
    /// checking.
    ///
    /// For a safe alternative see [`get_color`](Stamp::get_color) or
    /// [`get_color_checked`](Stamp::get_color_checked).
    ///
    /// # Safety
    ///
    /// Callers must ensure that the provided coordinate is within the bounds of the stamp.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stockbook::{stamp, Color, Size, Stamp};
    ///
    /// # macro_rules! stamp {
    /// #     ($path:literal) => { Stamp::<Size<3, 3>>::from_raw(&[0b101_010_10, 0b1_0000000]) };
    /// # }
    /// static IMAGE: Stamp<Size<3, 3>> = stamp!("checkerboard_3x3.png");
    ///
    /// // SAFETY: provided coordinates are guaranteed to be within the bounds
    /// // of the stamp
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(0, 0) }, Color::White);
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(1, 0) }, Color::Black);
    /// assert_eq!(unsafe { IMAGE.get_color_unchecked(0, 1) }, Color::Black);
    /// ```
    pub unsafe fn get_color_unchecked(&self, x: usize, y: usize) -> Color {
        let idx = y * self.width() + x;
        let byte = self.data.get_unchecked(idx / 8);
        let mask = 0b10000000 >> (idx % 8);

        if byte & mask != 0 {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl<S: traits::Size> Stamp<S> {
    const fn bytes_count(pixel_count: usize) -> usize {
        let d = pixel_count / 8;
        let r = pixel_count % 8;

        if r > 0 {
            d + 1
        } else {
            d
        }
    }
}

/// Color of a pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Black (`#000000ff` or `rgba(0, 0, 0, 255)`)
    Black,
    /// White (`#ffffffff` or `rgba(255, 255, 255, 255)`)
    White,
}
