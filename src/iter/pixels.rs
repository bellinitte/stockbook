use crate::{Color, Stamp};
use core::iter::FusedIterator;

/// An iterator that yields all pixels of a [`Stamp`].
///
/// This `struct` is created by the [`pixels`](Stamp::pixels) method on [`Stamp`].
/// See its documentation for more details.
#[derive(Debug)]
pub struct Pixels<'a> {
    cursor: Cursor<'a>,
    cursor_back: CursorBack<'a>,
    remaining: usize,
}

impl<'a> Pixels<'a> {
    pub(crate) fn new(stamp: &'a Stamp) -> Self {
        Self {
            cursor: Cursor::new(stamp),
            cursor_back: CursorBack::new(stamp),
            remaining: stamp.pixel_count(),
        }
    }
}

/// An iterator that cycles throygh all pixels of a [`Stamp`] from front to back.
#[derive(Debug)]
struct Cursor<'a> {
    x: usize,
    y: usize,
    stamp: &'a Stamp,
}

impl<'a> Cursor<'a> {
    fn new(stamp: &'a Stamp) -> Self {
        Self { x: 0, y: 0, stamp }
    }
}

impl Iterator for Cursor<'_> {
    type Item = (usize, usize, Color);

    fn next(&mut self) -> Option<(usize, usize, Color)> {
        let color = self.stamp.get_color_checked(self.x, self.y)?;
        let res = (self.x, self.y, color);

        self.x += 1;
        if self.x == self.stamp.width() {
            self.x = 0;
            self.y += 1;
            if self.y == self.stamp.height() {
                self.y = 0;
            }
        }

        Some(res)
    }
}

/// An iterator that cycles throygh all pixels of a [`Stamp`] from back to front.
#[derive(Debug)]
struct CursorBack<'a> {
    x: usize,
    y: usize,
    stamp: &'a Stamp,
}

impl<'a> CursorBack<'a> {
    fn new(stamp: &'a Stamp) -> Self {
        Self {
            x: stamp.width().saturating_sub(1),
            y: stamp.height().saturating_sub(1),
            stamp,
        }
    }
}

impl Iterator for CursorBack<'_> {
    type Item = (usize, usize, Color);

    fn next(&mut self) -> Option<(usize, usize, Color)> {
        let color = self.stamp.get_color_checked(self.x, self.y)?;
        let res = (self.x, self.y, color);

        match self.x.checked_sub(1) {
            Some(x) => self.x = x,
            None => {
                self.x = self.stamp.width().saturating_sub(1);
                match self.y.checked_sub(1) {
                    Some(y) => self.y = y,
                    None => self.y = self.stamp.height().saturating_sub(1),
                }
            }
        }

        Some(res)
    }
}

impl Iterator for Pixels<'_> {
    type Item = (usize, usize, Color);

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.checked_sub(1)?;
        self.cursor.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl DoubleEndedIterator for Pixels<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.checked_sub(1)?;
        self.cursor_back.next()
    }
}

impl ExactSizeIterator for Pixels<'_> {}

impl FusedIterator for Pixels<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_size_stamp() {
        let stamp = Stamp::from_raw(0, 0, &[]);
        let mut pixels = stamp.pixels();

        assert_eq!(pixels.next(), None);
    }

    #[test]
    fn test_zero_width_stamp() {
        let stamp = Stamp::from_raw(0, 3, &[]);
        let mut pixels = stamp.pixels();

        assert_eq!(pixels.next(), None);
    }

    #[test]
    fn test_zero_height_stamp() {
        let stamp = Stamp::from_raw(3, 0, &[]);
        let mut pixels = stamp.pixels();

        assert_eq!(pixels.next(), None);
    }

    #[test]
    fn test_double_ended() {
        let stamp = Stamp::from_raw(2, 2, &[0b1010_0000]);
        let mut pixels = stamp.pixels();

        assert_eq!(pixels.next(), Some((0, 0, Color::White)));
        assert_eq!(pixels.next_back(), Some((1, 1, Color::Black)));
        assert_eq!(pixels.next_back(), Some((0, 1, Color::White)));
        assert_eq!(pixels.next(), Some((1, 0, Color::Black)));
        assert_eq!(pixels.next(), None);
        assert_eq!(pixels.next_back(), None);
    }

    #[test]
    fn test_rev() {
        let stamp = Stamp::from_raw(2, 2, &[0b1010_0000]);
        let mut pixels = stamp.pixels().rev();

        assert_eq!(pixels.next(), Some((1, 1, Color::Black)));
        assert_eq!(pixels.next(), Some((0, 1, Color::White)));
        assert_eq!(pixels.next(), Some((1, 0, Color::Black)));
        assert_eq!(pixels.next(), Some((0, 0, Color::White)));
        assert_eq!(pixels.next(), None);
    }
}
