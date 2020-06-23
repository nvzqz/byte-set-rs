use super::ByteSet;

/// Operations related to the ASCII character set.
impl ByteSet {
    /// The set of all ASCII characters: U+0000 NULL ..= U+007F DELETE.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii`] returns `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII {
    ///     assert!(byte.is_ascii());
    /// }
    ///
    /// for byte in !ByteSet::ASCII {
    ///     assert!(!byte.is_ascii());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii
    pub const ASCII: Self = {
        #[cfg(target_pointer_width = "64")]
        {
            Self([!0, !0, 0, 0])
        }

        #[cfg(not(target_pointer_width = "64"))]
        {
            Self([!0, !0, !0, !0, 0, 0, 0, 0])
        }
    };

    /// The set of all ASCII alphabetic characters:
    ///
    /// - U+0041 'A' ..= U+005A 'Z'
    /// - U+0061 'a' ..= U+007A 'z'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_alphabetic`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_ALPHABETIC {
    ///     assert!(byte.is_ascii_alphabetic());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_ALPHABETIC {
    ///     assert!(!byte.is_ascii_alphabetic());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_alphabetic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
    pub const ASCII_ALPHABETIC: Self =
        Self::ASCII_LOWERCASE.inserting_all(Self::ASCII_UPPERCASE);

    /// The set of all ASCII uppercase characters: U+0041 'A' ..= U+005A 'Z'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_uppercase`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_UPPERCASE {
    ///     assert!(byte.is_ascii_uppercase());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_UPPERCASE {
    ///     assert!(!byte.is_ascii_uppercase());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_uppercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase
    pub const ASCII_UPPERCASE: Self = Self::from_range_inclusive(b'A'..=b'Z');

    /// The set of all ASCII lowercase characters: U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_lowercase`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_LOWERCASE {
    ///     assert!(byte.is_ascii_lowercase());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_LOWERCASE {
    ///     assert!(!byte.is_ascii_lowercase());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_lowercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase
    pub const ASCII_LOWERCASE: Self = Self::from_range_inclusive(b'a'..=b'z');

    /// The set of all ASCII alphanumeric characters:
    ///
    /// - U+0041 'A' ..= U+005A 'Z'
    /// - U+0061 'a' ..= U+007A 'z'
    /// - U+0030 '0' ..= U+0039 '9'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_alphanumeric`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_ALPHANUMERIC {
    ///     assert!(byte.is_ascii_alphanumeric());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_ALPHANUMERIC {
    ///     assert!(!byte.is_ascii_alphanumeric());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_alphanumeric`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphanumeric
    pub const ASCII_ALPHANUMERIC: Self =
        Self::ASCII_ALPHABETIC.inserting_all(Self::ASCII_DIGIT);

    /// The set of all ASCII decimal digits: U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_digit`] returns `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_DIGIT {
    ///     assert!(byte.is_ascii_digit());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_DIGIT {
    ///     assert!(!byte.is_ascii_digit());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_digit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_digit
    pub const ASCII_DIGIT: Self = Self::from_range_inclusive(b'0'..=b'9');

    /// The set of all ASCII hexadecimal digits:
    ///
    /// - U+0030 '0' ..= U+0039 '9'
    /// - U+0041 'A' ..= U+0046 'F'
    /// - U+0061 'a' ..= U+0066 'f'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_hexdigit`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_HEXDIGIT {
    ///     assert!(byte.is_ascii_hexdigit());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_HEXDIGIT {
    ///     assert!(!byte.is_ascii_hexdigit());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_hexdigit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_hexdigit
    pub const ASCII_HEXDIGIT: Self = Self::ASCII_DIGIT
        .inserting_all(Self::from_range_inclusive(b'A'..=b'F'))
        .inserting_all(Self::from_range_inclusive(b'a'..=b'f'));

    /// The set of all ASCII punctuation characters:
    ///
    /// - U+0021 ..= U+002F `! " # $ % & ' ( ) * + , - . /`
    /// - U+003A ..= U+0040 `: ; < = > ? @`
    /// - U+005B ..= U+0060 ``[ \ ] ^ _ ` ``
    /// - U+007B ..= U+007E `{ | } ~`
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_punctuation`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_PUNCTUATION {
    ///     assert!(byte.is_ascii_punctuation());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_PUNCTUATION {
    ///     assert!(!byte.is_ascii_punctuation());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_punctuation`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_punctuation
    pub const ASCII_PUNCTUATION: Self = byte_set![
        b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+',
        b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[',
        b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~',
    ];

    /// The set of all ASCII graphic characters: U+0021 '!' ..= U+007E '~'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_graphic`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_GRAPHIC {
    ///     assert!(byte.is_ascii_graphic());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_GRAPHIC {
    ///     assert!(!byte.is_ascii_graphic());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_graphic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_graphic
    pub const ASCII_GRAPHIC: Self =
        Self::ASCII_ALPHANUMERIC.inserting_all(Self::ASCII_PUNCTUATION);

    /// The set of all ASCII whitespace characters:
    ///
    /// - U+0020 SPACE
    /// - U+0009 HORIZONTAL TAB
    /// - U+000A LINE FEED
    /// - U+000C FORM FEED
    /// - U+000D CARRIAGE RETURN
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_whitespace`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_WHITESPACE {
    ///     assert!(byte.is_ascii_whitespace());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_WHITESPACE {
    ///     assert!(!byte.is_ascii_whitespace());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    pub const ASCII_WHITESPACE: Self =
        byte_set![b'\t', b'\n', 0x0C, b'\r', b' '];

    /// The set of all ASCII control characters:
    ///
    /// - U+0000 NUL ..= U+001F UNIT SEPARATOR
    /// - U+007F DELETE.
    ///
    /// Note that most ASCII whitespace characters are control characters, but
    /// SPACE is not.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_control`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_CONTROL {
    ///     assert!(byte.is_ascii_control());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_CONTROL {
    ///     assert!(!byte.is_ascii_control());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    pub const ASCII_CONTROL: Self =
        Self::from_range_inclusive(0..=0x1F).inserting(0x7F);

    /// Returns `true` if [`u8::is_ascii`] returns `true` for all bytes in
    /// `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii`]:
    /// https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii
    #[inline]
    #[must_use]
    pub const fn is_ascii(&self) -> bool {
        self._is_subset(&Self::ASCII)
    }

    /// Returns `true` if [`u8::is_ascii_alphabetic`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_alphabetic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphabetic(&self) -> bool {
        self._is_subset(&Self::ASCII_ALPHABETIC)
    }

    /// Returns `true` if [`u8::is_ascii_uppercase`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_uppercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase
    #[inline]
    #[must_use]
    pub const fn is_ascii_uppercase(&self) -> bool {
        self._is_subset(&Self::ASCII_UPPERCASE)
    }

    /// Returns `true` if [`u8::is_ascii_lowercase`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_lowercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase
    #[inline]
    #[must_use]
    pub const fn is_ascii_lowercase(&self) -> bool {
        self._is_subset(&Self::ASCII_LOWERCASE)
    }

    /// Returns `true` if [`u8::is_ascii_alphanumeric`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_alphanumeric`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphanumeric
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphanumeric(&self) -> bool {
        self._is_subset(&Self::ASCII_ALPHANUMERIC)
    }

    /// Returns `true` if [`u8::is_ascii_digit`] returns `true` for all bytes in
    /// `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_digit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_digit
    #[inline]
    #[must_use]
    pub const fn is_ascii_digit(&self) -> bool {
        self._is_subset(&Self::ASCII_DIGIT)
    }

    /// Returns `true` if [`u8::is_ascii_hexdigit`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_hexdigit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_hexdigit
    #[inline]
    #[must_use]
    pub const fn is_ascii_hexdigit(&self) -> bool {
        self._is_subset(&Self::ASCII_HEXDIGIT)
    }

    /// Returns `true` if [`u8::is_ascii_punctuation`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_punctuation`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_punctuation
    #[inline]
    #[must_use]
    pub const fn is_ascii_punctuation(&self) -> bool {
        self._is_subset(&Self::ASCII_PUNCTUATION)
    }

    /// Returns `true` if [`u8::is_ascii_graphic`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_graphic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_graphic
    #[inline]
    #[must_use]
    pub const fn is_ascii_graphic(&self) -> bool {
        self._is_subset(&Self::ASCII_GRAPHIC)
    }

    /// Returns `true` if [`u8::is_ascii_whitespace`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    #[inline]
    #[must_use]
    pub const fn is_ascii_whitespace(&self) -> bool {
        self._is_subset(&Self::ASCII_WHITESPACE)
    }

    /// Returns `true` if [`u8::is_ascii_control`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_control`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_control
    #[inline]
    #[must_use]
    pub const fn is_ascii_control(&self) -> bool {
        self._is_subset(&Self::ASCII_CONTROL)
    }
}
