/// A const subset of [`str::replace`],
/// which takes constants as arguments and returns a `&'static str`.
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait Pattern {}
///
/// fn str_replace(
///     string: &'static str,
///     pattern: impl Pattern,
///     replace_with: &'static str,
/// ) -> &'static str
/// # {""}
/// ```
/// Where `pattern` can be any of these types:
///
/// - `&'static str`
///
/// - `u8`: required to be ascii (`0` up to `127` inclusive).
///
/// # Example
///
///
/// ```rust
/// use const_format::str_replace;
///
/// // Passing a string pattern
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", "i", "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Passing an ascii u8 pattern.
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", b'i', "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Removing all instances of the pattern
/// assert_eq!(
///     str_replace!("remove haire", "re", ""),
///     "move hai",
/// );
///
/// // This shows that all the arguments can be `const`s, they don't have to be literals.
/// {
///     const IN: &str = "Foo Boo Patoo";
///     const REPLACING: &str = "oo";
///     const REPLACE_WITH: &str = "uh";
///     assert_eq!(str_replace!(IN, REPLACING, REPLACE_WITH), "Fuh Buh Patuh");
/// }
/// ```
///
/// [`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
#[macro_export]
#[cfg(feature = "const_generics")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
macro_rules! str_replace {
    ($string:expr, $replace:expr, $with:expr $(,)*) => {{
        const STR_OSRCTFL4A: &$crate::pmr::str = $string;

        const REPLACE_OSRCTFL4A: $crate::__str_methods::ReplaceInput =
            $crate::__str_methods::ReplaceInputConv($replace).conv();

        const WITH_OSRCTFL4A: &$crate::pmr::str = $with;

        {
            use $crate::__str_methods::{str_replace, str_replace_length};

            const L: $crate::pmr::usize =
                str_replace_length(STR_OSRCTFL4A, REPLACE_OSRCTFL4A, WITH_OSRCTFL4A);

            const OB: &[$crate::pmr::u8; L] =
                &str_replace::<L>(STR_OSRCTFL4A, REPLACE_OSRCTFL4A, WITH_OSRCTFL4A);

            const OS: &$crate::pmr::str = unsafe { $crate::__priv_transmute_bytes_to_str!(OB) };

            OS
        }
    }};
}

/// Creates a `&'static str` by repeating a `&'static str` some amount of times times.
///
/// # Example
///
/// ```rust
/// use const_format::str_repeat;
///
/// {
///     const OUT: &str = str_repeat!("hi ", 4);
///     assert_eq!(OUT, "hi hi hi hi ")
/// }
/// {
///     const IN: &str = "bye ";
///     const REPEAT: usize = 5;
///     const OUT: &str = str_repeat!(IN, REPEAT);
///     assert_eq!(OUT, "bye bye bye bye bye ")
/// }
///
/// ```
///
/// ### Failing
///
/// If this macro would produce too large a string,
/// it causes a compile-time error.
///
/// ```compile_fail
/// const_format::str_repeat!("hello", usize::MAX / 4);
/// ```
///
#[cfg_attr(
    feature = "testing",
    doc = r##"
```rust
const_format::str_repeat!("hello", usize::MAX.wrapping_add(4));
```
"##
)]
#[macro_export]
macro_rules! str_repeat {
    ($string:expr, $times:expr  $(,)*) => {{
        const STR_OSRCTFL4A: &$crate::pmr::str = $string;

        const TIMES_OSRCTFL4A: $crate::pmr::usize = $times;

        {
            use $crate::pmr::transmute;
            use $crate::pmr::{str, u8, usize};

            const STR_LEN: usize = STR_OSRCTFL4A.len();
            const OUT_LEN: usize = STR_LEN * TIMES_OSRCTFL4A;
            const OUT_B: &[u8; OUT_LEN] = &unsafe {
                let ptr = STR_OSRCTFL4A.as_ptr();
                transmute::<[[u8; STR_LEN]; TIMES_OSRCTFL4A], [u8; OUT_LEN]>(
                    [*transmute::<*const u8, &[u8; STR_LEN]>(ptr); TIMES_OSRCTFL4A],
                )
            };
            const OUT_S: &str = unsafe { $crate::__priv_transmute_bytes_to_str!(OUT_B) };
            OUT_S
        }
    }};
}

/// Replaces a substring in a `&'static str` constant.
/// Returns both the new resulting `&'static str`, and the replaced substring.
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
///
/// fn str_splice(
///     input: &'static str,
///     range: impl SomeIndex,
///     replace_with: &'static str,
/// ) -> const_format::SplicedStr
/// # {unimplemented!()}
/// ```
///
/// ### `range` argument
///
/// The `range` parameter determines what part of `input` is replaced,
/// and can be any of these types:
///
/// - `usize`: the starting index of a char, only includes that char.
/// - `Range<usize>`
/// - `RangeTo<usize>`
/// - `RangeFrom<usize>`
/// - `RangeInclusive<usize>`
/// - `RangeToInclusive<usize>`
/// - `RangeFull`
///
/// [`SplicedStr`] contains:
/// - `output`: a `&'static str` with the substring at `range` in `input` replaced with
/// `replace_with`.
/// - `removed`: the substring at `range` in `input`.
///
/// # Example
///
/// ```rust
/// use const_format::{str_splice, SplicedStr};
///
/// const OUT: SplicedStr = str_splice!("foo bar baz", 4..=6, "is");
/// assert_eq!(OUT , SplicedStr{output: "foo is baz", removed: "bar"});
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "this is bad";
///     const INDEX: std::ops::RangeFrom<usize> = 8..;
///     const REPLACE_WITH: &str = "... fine";
///     const OUT: SplicedStr = str_splice!(IN, INDEX, REPLACE_WITH);
///     assert_eq!(OUT , SplicedStr{output: "this is ... fine", removed: "bad"});
/// }
/// {
///     const OUT: SplicedStr = str_splice!("ABC豆-", 3, "DEFGH");
///     assert_eq!(OUT , SplicedStr{output: "ABCDEFGH-", removed: "豆"});
/// }
/// ```
///
/// ### Invalid index
///
/// Invalid indices cause compilation errors.
///
/// ```compile_fail
/// const_format::str_splice!("foo", 0..10, "");
/// ```
#[cfg_attr(
    feature = "testing",
    doc = r#"
```rust
const_format::str_splice!("foo", 0..3, "");
```

```compile_fail
const_format::str_splice!("foo", 0..usize::MAX, "");
```

```rust
assert_eq!(
    const_format::str_splice!("効率的", 3..6, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

```compile_fail
assert_eq!(
    const_format::str_splice!("効率的", 1..6, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

```compile_fail
assert_eq!(
    const_format::str_splice!("効率的", 3..5, "A"),
    const_format::SplicedStr{output: "効A的", removed: "率"} ,
);
```

"#
)]
///
///
/// [`SplicedStr`]: ./struct.SplicedStr.html
#[macro_export]
macro_rules! str_splice {
    ($string:expr, $index:expr, $insert:expr $(,)*) => {{
        const P_OSRCTFL4A: $crate::__str_methods::StrSpliceArgs =
            $crate::__str_methods::StrSplceArgsConv($string, $index, $insert).conv();
        {
            use $crate::__hidden_utils::PtrToRef;
            use $crate::__str_methods::{DecomposedString, SplicedStr, StrSpliceArgs};
            use $crate::pmr::{str, transmute, u8};

            const P: &StrSpliceArgs = &P_OSRCTFL4A;

            type DecompIn =
                DecomposedString<[u8; P.used_rstart], [u8; P.used_rlen], [u8; P.suffix_len]>;

            type DecompOut =
                DecomposedString<[u8; P.used_rstart], [u8; P.insert_len], [u8; P.suffix_len]>;

            $crate::pmr::respan_to! {
                ($string)
                const _ASSERT_VALID_INDEX: () = P.index_validity.assert_valid();
            }

            const OUT_A: (&DecompOut, &str) = unsafe {
                let input = PtrToRef {
                    ptr: P.str.as_ptr() as *const DecompIn,
                }
                .reff;
                let insert = PtrToRef {
                    ptr: P.insert.as_ptr() as *const [u8; P.insert_len],
                }
                .reff;

                (
                    &DecomposedString {
                        prefix: input.prefix,
                        middle: *insert,
                        suffix: input.suffix,
                    },
                    $crate::__priv_transmute_bytes_to_str!(&input.middle),
                )
            };

            const OUT: SplicedStr = unsafe {
                let output = OUT_A.0 as *const DecompOut as *const [u8; P.out_len];
                SplicedStr {
                    output: $crate::__priv_transmute_raw_bytes_to_str!(output),
                    removed: OUT_A.1,
                }
            };

            OUT
        }
    }};
}

/// Indexes a `&'static str` constant.
///
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
/// fn str_index(input: &'static str, range: impl SomeIndex) -> &'static str
/// # {unimplemented!()}
/// ```
/// This accepts
/// [the same index arguments as `str_splice`](macro.str_splice.html#range-argument)
///
/// # Example
///
/// ```
/// use const_format::str_index;
///
/// use std::ops::RangeFrom;
///
/// assert_eq!(str_index!("foo bar baz", ..7), "foo bar");
/// assert_eq!(str_index!("foo bar baz", 4..7), "bar");
/// assert_eq!(str_index!("foo bar baz", 4..), "bar baz");
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "hello world";
///     const INDEX_0: RangeFrom<usize> = 6..;
///     const OUT_0: &str = str_index!(IN, INDEX_0);
///     assert_eq!(OUT_0, "world");
///     
///     const INDEX_1: usize = 4;
///     const OUT_1: &str = str_index!(IN, INDEX_1);
///     assert_eq!(OUT_1, "o");
/// }
///
/// ```
///
/// ### Invalid index
///
/// Invalid indices cause compilation errors.
///
/// ```compile_fail
/// const_format::str_index!("foo", 0..10);
/// ```
#[cfg_attr(
    feature = "testing",
    doc = r#"
```rust
assert_eq!(const_format::str_index!("効率的", 3..6), "率");
```

```compile_fail
assert_eq!(const_format::str_index!("効率的", 3..5), "率");
```
```compile_fail
assert_eq!(const_format::str_index!("効率的", 4..6), "率");
```
"#
)]
///
///
#[macro_export]
macro_rules! str_index {
    ($string:expr, $index:expr $(,)*) => {{
        const P_OSRCTFL4A: $crate::__str_methods::StrIndexArgs =
            $crate::__str_methods::StrIndexArgsConv($string, $index).conv();

        {
            $crate::pmr::respan_to! {
                ($string)
                const _ASSERT_VALID_INDEX: () =
                    P_OSRCTFL4A.index_validity.assert_valid();
            }

            use $crate::__hidden_utils::PtrToRef;
            use $crate::__str_methods::DecomposedString;
            type DecompIn = DecomposedString<
                [u8; P_OSRCTFL4A.used_rstart],
                [u8; P_OSRCTFL4A.used_rlen],
                [u8; 0],
            >;

            const OUT: &'static str = unsafe {
                let input = PtrToRef {
                    ptr: P_OSRCTFL4A.str.as_ptr() as *const DecompIn,
                }
                .reff;
                $crate::__priv_transmute_raw_bytes_to_str!(&input.middle)
            };

            OUT
        }
    }};
}

/// Indexes a `&'static str` constant, returning `None` when the index is out of bounds.
///
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
/// fn str_get(input: &'static str, range: impl SomeIndex) -> Option<&'static str>
/// # {unimplemented!()}
/// ```
/// This accepts
/// [the same index arguments as `str_splice`](macro.str_splice.html#range-argument)
///
/// # Example
///
/// ```
/// use const_format::str_get;
///
/// use std::ops::RangeFrom;
///
/// assert_eq!(str_get!("foo bar baz", ..7), Some("foo bar"));
/// assert_eq!(str_get!("foo bar baz", 4..7), Some("bar"));
/// assert_eq!(str_get!("foo bar baz", 4..100), None);
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "hello world";
///     const INDEX_0: RangeFrom<usize> = 6..;
///     const OUT_0: Option<&str> = str_get!(IN, INDEX_0);
///     assert_eq!(OUT_0, Some("world"));
///     
///     const INDEX_1: usize = 4;
///     const OUT_1: Option<&str> = str_get!(IN, INDEX_1);
///     assert_eq!(OUT_1, Some("o"));
///
///     const INDEX_2: usize = IN.len();
///     const OUT_2: Option<&str> = str_get!(IN, INDEX_2);
///     assert_eq!(OUT_2, None);
/// }
///
/// ```
#[cfg_attr(
    feature = "testing",
    doc = r#"
```rust
assert_eq!(const_format::str_get!("効率的", 3..6), Some("率"));
assert_eq!(const_format::str_get!("効率的", 3..5), None);
assert_eq!(const_format::str_get!("効率的", 4..6), None);
```
"#
)]
///
#[macro_export]
macro_rules! str_get {
    ($string:expr, $index:expr $(,)*) => {{
        const P_OSRCTFL4A: $crate::__str_methods::StrIndexArgs =
            $crate::__str_methods::StrIndexArgsConv($string, $index).conv();

        {
            use $crate::__hidden_utils::PtrToRef;
            use $crate::__str_methods::DecomposedString;
            type DecompIn = DecomposedString<
                [u8; P_OSRCTFL4A.used_rstart],
                [u8; P_OSRCTFL4A.used_rlen],
                [u8; 0],
            >;

            const OUT: $crate::pmr::Option<&'static $crate::pmr::str> = unsafe {
                if P_OSRCTFL4A.index_validity.is_valid() {
                    let input = PtrToRef {
                        ptr: P_OSRCTFL4A.str.as_ptr() as *const DecompIn,
                    }
                    .reff;

                    $crate::pmr::Some($crate::__priv_transmute_raw_bytes_to_str!(&input.middle))
                } else {
                    $crate::pmr::None
                }
            };

            OUT
        }
    }};
}
