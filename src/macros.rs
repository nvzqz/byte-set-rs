/// Creates a [`ByteSet`] from a sequence of [`u8`]s.
///
/// `byte_set!` allows `ByteSet`s to be defined with the same syntax as [`vec!`]
/// or array expressions.
///
/// # Examples
///
/// This can be used within a `const` context:
///
/// ```
/// # use byte_set::{byte_set, ByteSet};
/// const SET: ByteSet = byte_set!(1, 2, 3, b'a', b'b', b'c');
///
/// assert!(SET.contains(b'a'));
/// ```
///
/// [`ByteSet`]: struct.ByteSet.html
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
/// [`vec!`]: https://doc.rust-lang.org/std/macro.vec.html
#[macro_export]
macro_rules! byte_set {
    ($($byte:expr,)*) => {
        $crate::ByteSet::new() $(.inserting($byte))*
    };
    ($($byte:expr),*) => {
        $crate::byte_set!($($byte,)*)
    };
}

/// Performs a `$map` operation over the `usize` slots of `$this` and `$other`,
/// combining the resulting `usize` slots with `$reduce`.
macro_rules! map_reduce_slots {
    ($this:expr, $other:expr, $map:tt, $reduce:tt) => {{
        // TODO: Might be worth wrapping a `const fn`? This is only being used
        // by binary ops, so this is fine for now.
        #[cfg(target_pointer_width = "64")]
        {
            ($this.0[0] $map $other.0[0]) $reduce
            ($this.0[1] $map $other.0[1]) $reduce
            ($this.0[2] $map $other.0[2]) $reduce
            ($this.0[3] $map $other.0[3])
        }

        #[cfg(target_pointer_width = "32")]
        {
            ($this.0[0] $map $other.0[0]) $reduce
            ($this.0[1] $map $other.0[1]) $reduce
            ($this.0[2] $map $other.0[2]) $reduce
            ($this.0[3] $map $other.0[3]) $reduce
            ($this.0[4] $map $other.0[4]) $reduce
            ($this.0[5] $map $other.0[5]) $reduce
            ($this.0[6] $map $other.0[6]) $reduce
            ($this.0[7] $map $other.0[7])
        }
    }};
}

/// Performs a `$map` operation over the `usize` slots of `$this` and `$other`,
/// returning the resulting `ByteSet`.
macro_rules! map_slots {
    ($this:expr, $map:tt) => {{
        // TODO: Might be worth wrapping a `const fn`? This is only being used
        // by `!`, so being a prefix op is fine for now.
        #[cfg(target_pointer_width = "64")]
        {
            ByteSet([
                $map $this.0[0], $map $this.0[1],
                $map $this.0[2], $map $this.0[3],
            ])
        }

        #[cfg(target_pointer_width = "32")]
        {
            ByteSet([
                $map $this.0[0], $map $this.0[1],
                $map $this.0[2], $map $this.0[3],
                $map $this.0[4], $map $this.0[5],
                $map $this.0[6], $map $this.0[7],
            ])
        }
    }};
    ($this:expr, $map:tt, $other:expr) => {{
        #[cfg(target_pointer_width = "64")]
        {
            ByteSet([
                ($this.0[0] $map $other.0[0]), ($this.0[1] $map $other.0[1]),
                ($this.0[2] $map $other.0[2]), ($this.0[3] $map $other.0[3]),
            ])
        }

        #[cfg(target_pointer_width = "32")]
        {
            ByteSet([
                ($this.0[0] $map $other.0[0]), ($this.0[1] $map $other.0[1]),
                ($this.0[2] $map $other.0[2]), ($this.0[3] $map $other.0[3]),
                ($this.0[4] $map $other.0[4]), ($this.0[5] $map $other.0[5]),
                ($this.0[6] $map $other.0[6]), ($this.0[7] $map $other.0[7]),
            ])
        }
    }};
}
