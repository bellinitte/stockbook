use stockbook::{stamp, Color, Size, Stamp};

static STAMP: Stamp<Size<2, 2>> = stamp!("tests/assets/checkerboard_2x2.png");

#[test]
fn integration() {
    assert_eq!(STAMP.width(), 2);
    assert_eq!(STAMP.height(), 2);
    assert_eq!(STAMP.size(), [2, 2]);

    assert!(STAMP.is_within_bounds(0, 0));
    assert!(STAMP.is_within_bounds(1, 1));
    assert!(!STAMP.is_within_bounds(2, 1));
    assert!(!STAMP.is_within_bounds(1, 2));

    assert_eq!(STAMP.get_color(0, 0), Color::White);
    assert_eq!(STAMP.get_color(1, 0), Color::Black);
    assert_eq!(STAMP.get_color(0, 1), Color::Black);
    assert_eq!(STAMP.get_color(1, 1), Color::White);

    assert_eq!(STAMP.get_color_checked(0, 0), Some(Color::White));
    assert_eq!(STAMP.get_color_checked(1, 0), Some(Color::Black));
    assert_eq!(STAMP.get_color_checked(0, 1), Some(Color::Black));
    assert_eq!(STAMP.get_color_checked(1, 1), Some(Color::White));

    // SAFETY: coordinates are guaranteed to be within the bounds of the stamp
    unsafe {
        assert_eq!(STAMP.get_color_unchecked(0, 0), Color::White);
        assert_eq!(STAMP.get_color_unchecked(1, 0), Color::Black);
        assert_eq!(STAMP.get_color_unchecked(0, 1), Color::Black);
        assert_eq!(STAMP.get_color_unchecked(1, 1), Color::White);
    }

    let mut pixels = STAMP.pixels();
    assert_eq!(pixels.next(), Some((0, 0, Color::White)));
    assert_eq!(pixels.next(), Some((1, 0, Color::Black)));
    assert_eq!(pixels.next(), Some((0, 1, Color::Black)));
    assert_eq!(pixels.next(), Some((1, 1, Color::White)));
    assert_eq!(pixels.next(), None);
}
