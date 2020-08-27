//! [`std::fmt`]-like api that can be used at compile-time.
//!
//! This module requires the "fmt" feature to be enabled, and the nightly compiler,
//! because at the time of writing these docs (2020-08-XX) mutable references in const fn
//! require the unstable
//! [`const_mut_refs`](https://github.com/rust-lang/rust/issues/57349) feature.
//!
//! # Implementing the formatting methods
//!
//! Users of this library can implement debug and display formatting by
//! defining `const_debug_fmt` and `const_display_fmt` inherent methods
//! with the
//! ```ignore
//! // use const_format::{Formatter, Error};
//! const fn const_debug_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
//! const fn const_display_fmt(&self, &mut Formatter<'_>) -> Result<(), Error>
//! ```
//! signatures,
//! and implementing the [`FormatMarker`] trait.
//!
//! # Limitations
//!
//! ### Generic impls
//!
//! Because the formatting of custom types is implemented with duck typing,
//! it's not possible to format generic types, instead you must do either of these:
//!
//! - Provide all the implementations ahead of time, what [`impl_fmt`] is for.
//!
//! - Provide a macro that formats the type.
//! The `call_debug_fmt` macro is a version of this that formats generic std types.
//!
//! # Formatting Syntax
//!
//! The formatting macros all share the formatting syntax,
//! modeled after the syntax of the formatting macros of the standard library.
//!
//! ### Argumments
//!
//! Arguments in the format string can be named and positional in these ways:
//!
//! - Implcitly positional(eg: `formatc!("{}", 20u8)`):<br>
//! Starts at the 0th positional argument and increments with every use.
//!
//! - Explicit positional(eg: `formatc!("{0}", 10u8)`).
//!
//! - Named, passed to the macro as named arguments (eg: `formatc!("{foo}", foo = 10u8)`).
//!
//! - Named, from constant (eg: `formatc!("{FOO}")`):
//! Uses the `FOO` constant from the enclosing scope scope.
//!
//! ### Formatters
//!
//! The format arguments can be formatted in these ways:
//!
//! - Debug formatting (eg: `formatc!("{:?}", 0u8)` ):<br>
//! Similar to how Debug formatting in the standard library works,
//! except that it does not escape unicode characters.
//!
//! - Display formatting (eg: `formatc!("{}", 0u8)`, `formatc!("{:}", 0u8)` )
//!
//! - Hexadecimal formatting (eg: `formatc!("{:x}", 0u8)`):
//! Writes numbers in capialized hexadecimal.
//! This can be combined with debug formatting, with the `"{:x?}"` formatter.
//!
//! - Binary formatting (eg: `formatc!("{:b}", 0u8)`):
//! This can be combined with debug formatting, with the `"{:b?}"` formatter.
//!
//! ### Alternate flag
//!
//! The alternate flag allows types to format themselves in an alternate way,
//! written as "#" in a format string argument. eg:`"{:#}", "{:#?}"`.
//!
//! This is the built-in behavior for the alternate flag:
//!
//! - The Debug formater (eg: `formatc!("{:#?}", FOO)`):
//! pretty print structs and enums.
//!
//! - The hexadecimal formater (eg: `formatc!("{:#x}", FOO)`):
//! prefixes numbers with `0x`.
//!
//! - The binary formater (eg: `formatc!("{:#b}", FOO)`):
//! prefixes numbers with `0b`.
//!
//!
//! # Examples
//!
//! ### Derive
//!
//! This example demonstrates how you can derive [`ConstDebug`], and use it with the `fmt` API.
//!
//! Ìt requires the "derive" feature to be enabled
//!
//! ```rust
//! #![feature(const_mut_refs)]
//!
//! use const_format::{Error, Formatter, FormattingFlags, PWrapper, StrWriter};
//! use const_format::{ConstDebug, try_, unwrap, writec};
//!
//! use std::ops::Range;
//!
//! #[derive(ConstDebug)]
//! pub struct Foo {
//!     range: Option<Range<usize>>,
//!     point: Point,
//! }
//!
//! #[derive(ConstDebug)]
//! pub struct Point {
//!     x: u32,
//!     y: u32,
//! }
//!
//! const CAP: usize = 90;
//! const fn build_string() -> StrWriter<[u8; CAP]> {
//!     let mut writer = StrWriter::new([0; CAP]);
//!
//!     let foo = Foo {
//!         range: Some(0..10),
//!         point: Point{ x: 13, y: 21 },
//!     };
//!
//!     unwrap!(writec!(writer, "{:x?}", foo));
//!
//!     writer
//! }
//!
//! const WRITER: &StrWriter<[u8]> = &build_string();
//!
//! // The formatter
//! assert_eq!(
//!     WRITER.as_str(),
//!     "Foo { range: Some(0..A), point: Point { x: D, y: 15 } }",
//! );
//!
//! ```
//!
//!
//! ### No proc macros
//!
//! This example demonstrates how you can use the `fmt` api without using any proc macros.
//!
//! ```rust
//! #![feature(const_mut_refs)]
//!
//! use const_format::{Error, Formatter, FormattingFlags, PWrapper, StrWriter};
//! use const_format::{call_debug_fmt, coerce_to_fmt, impl_fmt, try_};
//!
//! use std::cmp::Ordering;
//!
//! pub struct Foo<T, U> {
//!     a: u32,
//!     b: u32,
//!     c: T,
//!     d: [Ordering; 3],
//!     ignored: U,
//! }
//!
//! //
//! impl_fmt!{
//!     // The type parameters of the impl must be written with trailing commas
//!     impl[U,] Foo<u32, U>;
//!     impl[U,] Foo<&str, U>;
//!
//!     pub const fn const_debug_fmt(&mut self, f: &mut Formatter<'_>) -> Result<(), Error> {
//!         let mut f = f.debug_struct("Foo");
//!
//!         // PWrapper is a wrapper for std types, which defines the formatter methods for them.
//!         try_!(PWrapper(self.a).const_debug_fmt(f.field("a")));
//!
//!         try_!(PWrapper(self.b).const_debug_fmt(f.field("b")));
//!
//!         // The `coerce_to_fmt` macro automatically wraps std types in `PWrapper`
//!         // and does nothing with non-std types.
//!         try_!(coerce_to_fmt!(self.c).const_debug_fmt(f.field("c")));
//!
//!         // This macro allows debug formatting of some generic types which
//!         // wrap non-std types, including:
//!         // - arrays - slices - Option - newtype wrappers
//!         call_debug_fmt!(array, self.d, f.field("d"));
//!
//!         f.finish()
//!     }
//! }
//!
//! const CAP: usize = 128;
//!
//! const fn build_string() -> StrWriter<[u8; CAP]> {
//!     let flags = FormattingFlags::NEW.set_alternate(true);
//!     let mut writer = StrWriter::new([0; CAP]);
//!
//!     const_format::unwrap!(
//!         Foo {
//!             a: 5,
//!             b: 8,
//!             c: 13,
//!             d: [Ordering::Less, Ordering::Equal, Ordering::Greater],
//!             ignored: (),
//!         }.const_debug_fmt(&mut Formatter::from_sw(flags, &mut writer))
//!     );
//!
//!     writer
//! }
//!
//! const STRING: &StrWriter<[u8]> = &build_string();
//!
//! assert_eq!(
//!     STRING.as_str(),
//!     "\
//! Foo {
//!     a: 5,
//!     b: 8,
//!     c: 13,
//!     d: [
//!         Less,
//!         Equal,
//!         Greater,
//!     ],
//! }\
//!     ",
//! );
//!
//!
//! ```
//!
//! [`std::fmt`]: https://doc.rust-lang.org/std/fmt/
//!
//! [`FormatMarker`]: ../marker_traits/trait.FormatMarker.trait
//! [`ConstDebug`]: ../derive.ConstDebug.trait
//!
//!
//!
//!
//!

mod error;
mod formatter;
mod std_type_impls;
mod str_writer;
mod str_writer_mut;

pub use crate::formatting::{FormattingFlags, FormattingMode};

pub use self::{
    error::Error,
    formatter::{ComputeStrLength, DebugList, DebugSet, DebugStruct, DebugTuple, Formatter},
    str_writer::StrWriter,
    str_writer_mut::StrWriterMut,
};

#[cfg(all(test, not(feature = "only_new_tests")))]
mod tests;
