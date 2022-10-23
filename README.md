# stockbook

[![CI](https://github.com/karolbelina/stockbook/actions/workflows/ci.yml/badge.svg)](https://github.com/karolbelina/stockbook/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/stockbook.svg)](https://crates.io/crates/stockbook)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest-informational.svg)](https://docs.rs/stockbook)

Stockbook embeds 1-bit raster images in your code at compile time.

Designed primarily for `#![no_std]` usage, in embedded or other program-memory-constrained environments.

```toml
[dependencies]
stockbook = "0.1.1"
```

The main functionality of Stockbook is the `stamp!` macro, which lets you include data similarly to how [`include_bytes!`](https://doc.rust-lang.org/stable/core/macro.include_bytes.html) does, but from an image, specifically a 1-bit black and white image. The macro returns a `Stamp` type, which just holds a static reference to the pixel data &mdash; the size of the image is encoded statically in the type. The pixel data is represented internally as an array of bytes, in which individual bits correspond to individual pixels.

## Example

File `assets/star.png` (scaled x8 for preview, originally 12x12 px):

![Star](https://github.com/karolbelina/stockbook/blob/main/docs/star.png?raw=true)

File `src/lib.rs`:

```rust
use stockbook::{stamp, Color, Size, Stamp};

static STAR_SPRITE: Stamp<Size<12, 12>> = stamp!("assets/star.png");

pub fn draw_star() {
    for (x, y, color) in STAR_SPRITE.pixels() {
        match color {
            Color::Black => {}, // Treat as transparent
            Color::White => draw_pixel_at(x, y),
        }
    }
}

fn draw_pixel_at(x: usize, y: usize) {
    /* ... */
}
```

## Supported formats

Stockbook uses the [image](https://docs.rs/image) crate under the hood. See its own [list of supported formats](https://docs.rs/image/latest/image/codecs/index.html#supported-formats) for more details.

## Unstable features

Although this library works on `stable`, any changes to images referenced by the `stamp!` macro might not be detected because of caching. Therefore, until [`track_path` API](https://doc.rust-lang.org/stable/proc_macro/tracked_path/fn.path.html) ([Tracking Issue](https://github.com/rust-lang/rust/issues/99515)) stabilizes, it is recommended to use the `nightly` toolchain, however functionality behind this feature is unstable and may change or stop compiling at any time.

## License

This software is licensed under the MIT license.

See the [LICENSE](LICENSE) file for more details.
