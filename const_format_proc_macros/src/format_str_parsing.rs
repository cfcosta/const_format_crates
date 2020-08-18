use syn::Ident;

use std::str::FromStr;

mod errors;

#[cfg(test)]
mod tests;

pub(crate) use self::errors::{ParseError, ParseErrorKind};

#[derive(Debug, PartialEq)]
pub(crate) struct FormatStr {
    pub(crate) list: Vec<FmtStrComponent>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum FmtStrComponent {
    Str(String),
    Arg(FmtArg),
}

#[derive(Debug, PartialEq)]
pub(crate) struct FmtArg {
    pub(crate) which_arg: WhichArg,
    pub(crate) formatting: Formatting,
}

#[derive(Debug, PartialEq)]
pub(crate) enum WhichArg {
    Ident(syn::Ident),
    Positional(Option<usize>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Formatting {
    Debug,
    Display,
}

/////////////////////////////////////

#[allow(dead_code)]
impl FmtStrComponent {
    fn str(s: &str) -> Self {
        Self::Str(s.to_string())
    }
    fn arg(which_arg: WhichArg, formatting: Formatting) -> Self {
        Self::Arg(FmtArg {
            which_arg,
            formatting,
        })
    }
}

#[allow(dead_code)]
impl FmtArg {
    fn new(which_arg: WhichArg, formatting: Formatting) -> Self {
        Self {
            which_arg,
            formatting,
        }
    }
}

#[allow(dead_code)]
impl WhichArg {
    fn ident(s: &str) -> Self {
        Self::Ident(Ident::new(s, proc_macro2::Span::mixed_site()))
    }
}

/////////////////////////////////////

impl FromStr for FormatStr {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<FormatStr, ParseError> {
        parse_format_str(input)
    }
}

fn parse_format_str(input: &str) -> Result<FormatStr, ParseError> {
    let mut components = Vec::<FmtStrComponent>::new();

    let mut arg_start = 0;

    loop {
        let open_pos = input.find_from('{', arg_start);

        let str = &input[arg_start..open_pos.unwrap_or(input.len())];
        components.push_arg_str(parse_mid_str(str, arg_start)?);

        if let Some(open_pos) = open_pos {
            let after_open = open_pos + 1;
            if input[after_open..].chars().next() == Some('{') {
                components.push_arg_str("{".to_string());

                arg_start = open_pos + 2;
            } else if let Some(close_pos) = input.find_from('}', after_open) {
                let after_close = close_pos + 1;

                let arg = parse_fmt_arg(&input[after_open..close_pos], after_open)?;
                components.push(FmtStrComponent::Arg(arg));

                arg_start = after_close;
            } else {
                return Err(ParseError {
                    pos: open_pos,
                    kind: ParseErrorKind::UnclosedArg,
                });
            }
        } else {
            break;
        }
    }

    Ok(FormatStr { list: components })
}

/// Parses the text between arguments, to unescape `}}` into `}`
fn parse_mid_str(str: &str, starts_at: usize) -> Result<String, ParseError> {
    let mut buffer = String::with_capacity(str.len());

    let mut starts_pos = 0;
    let bytes = str.as_bytes();

    while let Some(close_pos) = str.find_from('}', starts_pos) {
        let after_close = close_pos + 1;
        if bytes.get(after_close) == Some(&b'}') {
            buffer.push_str(&str[starts_pos..after_close]);
            starts_pos = after_close + 1;
        } else {
            return Err(ParseError {
                pos: starts_at + close_pos,
                kind: ParseErrorKind::InvalidClosedArg,
            });
        }
    }
    buffer.push_str(&str[starts_pos..]);

    Ok(buffer)
}

/// Parses the format arguments (`{:?}`, `{foo:}`, `{0}`, etc).
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_fmt_arg(input: &str, starts_at: usize) -> Result<FmtArg, ParseError> {
    let colon = input.find(':');

    let which_arg_str = &input[..colon.unwrap_or(input.len())];
    let formatting_str = colon.map_or("", |x| &input[x + 1..]);
    let formatting_starts_at = colon.map_or(input.len(), |x| starts_at + x + 1);

    Ok(FmtArg::new(
        parse_which_arg(which_arg_str, starts_at)?,
        parse_formatting(formatting_str, formatting_starts_at)?,
    ))
}

/// Parses the name of the argument in `{foo}`, `{}`, `{bar:?}`
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_which_arg(input: &str, starts_at: usize) -> Result<WhichArg, ParseError> {
    if input.is_empty() {
        Ok(WhichArg::Positional(None))
    } else if input.as_bytes()[0].is_ascii_digit() {
        match input.parse::<usize>() {
            Ok(number) => Ok(WhichArg::Positional(Some(number))),
            Err(_) => Err(ParseError {
                pos: starts_at,
                kind: ParseErrorKind::NotANumber {
                    what: input.to_string(),
                },
            }),
        }
    } else {
        parse_ident(input, starts_at)
    }
}

/// Parses the `?` and other formatters inside formatting arguments (`{}`).
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_formatting(input: &str, starts_at: usize) -> Result<Formatting, ParseError> {
    match input {
        "" => Ok(Formatting::Display),
        "?" => Ok(Formatting::Debug),
        _ => Err(ParseError {
            pos: starts_at,
            kind: ParseErrorKind::UnknownFormatting {
                what: input.to_string(),
            },
        }),
    }
}

// Parses an identifier in a formatting argument.
// This panics if called with an empty string
///
/// `starts_at` is the offset of `input` in the formatting string.
fn parse_ident(ident_str: &str, starts_at: usize) -> Result<WhichArg, ParseError> {
    let mut ident_chars = ident_str.chars();

    let first = ident_chars.next().unwrap();

    if !first.is_alphabetic() && (first != '_' || ident_str.len() == 1)
        || ident_chars.any(|c| !c.is_alphanumeric() && c != '_')
    {
        Err(ParseError {
            pos: starts_at,
            kind: ParseErrorKind::NotAnIdent {
                what: ident_str.to_string(),
            },
        })
    } else {
        Ok(WhichArg::ident(ident_str))
    }
}

////////////////////////////////////////////////////////////////////////////////

trait VecExt {
    fn push_arg_str(&mut self, str: String);
}

impl VecExt for Vec<FmtStrComponent> {
    fn push_arg_str(&mut self, str: String) {
        if !str.is_empty() {
            self.push(FmtStrComponent::Str(str));
        }
    }
}

trait StrExt {
    fn find_from(&self, c: char, from: usize) -> Option<usize>;
}

impl StrExt for str {
    fn find_from(&self, c: char, from: usize) -> Option<usize> {
        self[from..].find(c).map(|p| p + from)
    }
}