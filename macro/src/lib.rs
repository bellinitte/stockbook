#![cfg_attr(use_unstable_features, feature(track_path))]

use image::{GenericImageView as _, Pixel as _};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::path::Path;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    parse_macro_input, LitStr,
};

/// Includes an image as a [`Stamp`][Stamp].
///
/// The provided path is interpreted in a platform-specific way at compile time. The
/// image’s format is determined from the path’s file extension.
///
/// The pixels of the image must be either black (`#000000ff` or
/// `rgba(0, 0, 0, 255)`) or white (`#ffffffff` or `rgba(255, 255, 255, 255)`). Any
/// other color will result in a compile-time error.
///
/// This macro will encode the image and yield an expression of type
/// [`Stamp`][Stamp] with the pixel data included.
///
/// If the `"progmem"` feature is enabled and the target architecture is set to
/// `avr`, the pixel data will be placed into the `.progmem.data` section using the
/// `#[link_section = ".progmem.data"]` attribute.
///
/// # Examples
///
/// Assume there are two files in the same directory: a 16x12 pixel image
/// `image.png`, and a `main.rs` with the following contents:
///
/// ```rust,ignore
/// use stockbook::{stamp, Stamp};
///
/// static IMAGE: Stamp = stamp!("image.png");
/// ```
///
/// Compiling `main.rs` is going to statically embed the image in the binary.
///
/// # Quirks
///
/// ## Input
///
/// Being a preprocessor/macro means that [`stamp!`] operates on the level of the
/// syntax and is expanded before the compiler interprets any other code, so the
/// only way to provide the path to the image is through a direct string literal.
///
/// For example, this fails to compile:
///
/// ```rust,ignore
/// use stockbook::{stamp, Stamp};
///
/// const IMAGE_PATH: &str = "image.png";
///
/// static IMAGE: Stamp = stamp!(IMAGE_PATH); // passing an identifier, not a string literal
/// ```
///
/// ## Relative paths
///
/// Note that unlike [`include_bytes!`], the file is not located relative to the
/// current file. Generally, when invoking [`stamp!`] you should try to avoid using
/// relative paths because `rustc` makes no guarantees about the current directory
/// when it is running a procedural macro.
///
/// [Stamp]: struct.Stamp.html
#[proc_macro]
pub fn stamp(input: TokenStream) -> TokenStream {
    let stamp = parse_macro_input!(input as Stamp);
    quote! { #stamp }.into()
}

struct Stamp {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[derive(Default, Clone, Copy)]
enum Color {
    #[default]
    Black,
    White,
}

impl Parse for Stamp {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit_str = input.parse::<LitStr>()?;
        let path_str = lit_str.value();
        let path = Path::new(&path_str);

        track_file_if_available(path);

        let img = image::open(path).map_err(|error| {
            Error::new(
                input.span(),
                format!("couldn't read {}: {}", path.display(), error),
            )
        })?;

        let (width, height) = img.dimensions();
        let (width, height) = (width as usize, height as usize);

        let mut colors = vec![Default::default(); width * height];

        for (x, y, color) in img.pixels() {
            let channels = color.channels();
            let [r, g, b, a] = [channels[0], channels[1], channels[2], channels[3]];

            let color = match [r, g, b, a] {
                [0, 0, 0, 255] => Color::Black,

                [255, 255, 255, 255] => Color::White,

                _ => {
                    return Err(Error::new(
                        input.span(),
                        format!(
                            "invalid pixel at {},{} (#{:02x}{:02x}{:02x}{:02x})",
                            x, y, r, g, b, a
                        ),
                    ))
                }
            };

            let index = y as usize * width + x as usize;
            colors[index] = color;
        }

        let mut data = vec![0u8; encoding_len(width * height)];

        for (index, color) in colors.iter().enumerate() {
            let byte_index = index / 8;
            let bit_index = 7 - (index % 8);
            let byte = &mut data[byte_index];

            match color {
                Color::Black => *byte &= !(1 << bit_index),
                Color::White => *byte |= 1 << bit_index,
            }
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

fn encoding_len(pixel_count: usize) -> usize {
    let d = pixel_count / 8;
    let r = pixel_count % 8;

    if r > 0 {
        d + 1
    } else {
        d
    }
}

fn track_file_if_available(path: impl AsRef<Path>) {
    #[cfg(use_unstable_features)]
    proc_macro::tracked_path::path(format!("{}", path.as_ref().display()));

    #[cfg(not(use_unstable_features))]
    let _ = path;
}

impl ToTokens for Stamp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let width = self.width;
        let height = self.height;
        let array_len = self.data.len();
        let array = syn::ExprArray {
            attrs: Default::default(),
            bracket_token: Default::default(),
            elems: self
                .data
                .iter()
                .map(|byte| {
                    syn::Expr::Lit(syn::ExprLit {
                        attrs: Default::default(),
                        lit: syn::Lit::Int(syn::LitInt::new(&byte.to_string(), Span::call_site())),
                    })
                })
                .collect(),
        };

        #[cfg(feature = "progmem")]
        let progmem_attr = quote! {
            #[cfg_attr(target_arch = "avr", link_section = ".progmem.data")]
        };
        #[cfg(not(feature = "progmem"))]
        let progmem_attr = TokenStream2::new();

        tokens.extend(quote! {
            {
                #progmem_attr
                static PIXEL_DATA: [u8; #array_len] = #array;

                unsafe {
                    ::stockbook::Stamp::from_raw(#width, #height, PIXEL_DATA.as_ptr())
                }
            }
        });
    }
}
