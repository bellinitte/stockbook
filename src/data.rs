#[cfg(feature = "progmem")]
use avr_progmem::{raw::read_byte, wrapper::ProgMem};
use cfg_if::cfg_if;
use core::fmt::Debug;

/// Byte array wrapper &mdash; source of data for a [`Stamp`](crate::Stamp).
pub struct Data {
    #[cfg(not(feature = "progmem"))]
    source: *const u8,

    #[cfg(feature = "progmem")]
    source: ProgMem<u8>,
}

impl Data {
    /// Constructs a new instance of this type.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a byte array.
    ///
    /// If the `"progmem"` feature is enabled, `ptr` must point to a valid byte array
    /// that is stored in the program memory domain. The array must be initialized,
    /// readable, and immutable (i.e. it must not be changed). Also the pointer must be
    /// valid for the `'static` lifetime.
    pub const unsafe fn from_raw(ptr: *const u8) -> Self {
        cfg_if! {
            if #[cfg(feature = "progmem")] {
                Self {
                    source: ProgMem::new(ptr),
                }
            } else {
                Self {
                    source: ptr,
                }
            }
        }
    }

    /// Returns a byte at `idx`, without doing bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior, even if
    /// the resulting reference is not used.
    pub unsafe fn get_unchecked(&self, idx: usize) -> u8 {
        let ptr = self.as_ptr().add(idx);
        Self::deref(ptr)
    }

    /// Return the raw pointer to the inner value.
    ///
    /// If the `"progmem"` feature is enabled, the returned pointer must not be
    /// dereferenced via the default Rust operations.
    pub fn as_ptr(&self) -> *const u8 {
        cfg_if! {
            if #[cfg(feature = "progmem")] {
                self.source.as_ptr()
            } else {
                self.source
            }
        }
    }

    unsafe fn deref(ptr: *const u8) -> u8 {
        cfg_if! {
            if #[cfg(feature = "progmem")] {
                // Since we're building with the `"progmem"` feature, `ptr` is valid in the program
                // domain.
                read_byte(ptr)
            } else {
                *ptr
            }
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:p}", self.as_ptr())
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        cfg_if! {
            if #[cfg(feature = "progmem")] {
                // SAFETY: we construct a `ProgMem` with a pointer we got from a `ProgMem`.
                // Required becase `ProgMem` doesn't provide a `Clone` implementation.
                let source = unsafe { ProgMem::new(self.source.as_ptr()) };
                Self {
                    source,
                }
            } else {
                Self {
                    source: self.source,
                }
            }
        }
    }
}

unsafe impl Send for Data {
    // SAFETY: pointers per-se are sound to send and share. Furthermore, we never mutate
    // the underling value, thus `Data` can be considered as some sort of a sharable
    // `'static` "reference". Thus it can be shared and transferred between threads.
}

unsafe impl Sync for Data {
    // SAFETY: pointers per-se are sound to send and share. Furthermore, we never mutate
    // the underling value, thus `Data` can be considered as some sort of a sharable
    // `'static` "reference". Thus it can be shared and transferred between threads.
}
