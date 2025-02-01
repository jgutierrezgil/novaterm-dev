//! ANSI sequence parsing and HTML conversion
//! 
//! This module provides functionality for:
//! - Parsing ANSI escape sequences
//! - Converting ANSI elements to HTML with CSS classes
//! - XSS-safe HTML generation
//! 
//! # Example
//! ```
//! use novaterm_core::ansi::{AnsiElement, HtmlConverter};
//! 
//! let mut converter = HtmlConverter::new("term".to_string());
//! 
//! // Convert red text
//! let red = AnsiElement::Csi {
//!     params: vec![38, 2, 255, 0, 0],
//!     intermediates: vec![],
//!     ignore: false,
//! };
//! let text = AnsiElement::Text("Hello".to_string());
//! let reset = AnsiElement::Csi {
//!     params: vec![0],
//!     intermediates: vec![],
//!     ignore: false,
//! };
//! 
//! // Generate HTML
//! let html = vec![red, text, reset].iter()
//!     .map(|elem| converter.convert(elem).unwrap())
//!     .collect::<String>();
//! 
//! // Get CSS
//! let css = converter.get_css();
//! ```

mod html;
pub use html::{HtmlConverter, HtmlError};

#[derive(Debug, PartialEq, Eq)]
pub enum AnsiElement {
    Text(String),
    Csi {
        params: Vec<i32>,
        intermediates: Vec<u8>,
        ignore: bool,
    },
    Osc(Vec<Vec<u8>>),
    Esc(Vec<u8>),
}

#[derive(Debug, thiserror::Error)]
pub enum AnsiError {
    #[error("Secuencia incompleta")]
    Incomplete,
    #[error("Carácter inválido: {0}")]
    InvalidChar(u8),
}

use nom::{branch::alt, bytes::complete::*, combinator::*, sequence::*, IResult};

fn parse_csi(input: &[u8]) -> IResult<&[u8], AnsiElement> {
    let (input, _) = tag(b"\x1b[")(input)?;
    let (input, (params, intermediates, ignore)) = tuple((
        separated_list0(tag(b";"), nom::character::complete::i32),
        take_while(|c| (0x20..=0x2F).contains(&c)),
        opt(tag(b"?")).map(|o| o.is_some()),
    ))(input)?;
    let (input, cmd) = anychar(input)?;
    
    Ok((input, AnsiElement::Csi {
        params,
        intermediates: intermediates.to_vec(),
        ignore,
    }))
}

fn parse_osc(input: &[u8]) -> IResult<&[u8], AnsiElement> {
    let (input, _) = tag(b"\x1b]")(input)?;
    let (input, args) = separated_list0(tag(b";"), take_until(b"\x07"))(input)?;
    let (input, _) = tag(b"\x07")(input)?;
    
    Ok((input, AnsiElement::Osc(args.iter().map(|&a| a.to_vec()).collect())))
}

#[test]
fn test_csi_color() {
    let input = b"\x1b[38;2;255;0;0m";
    let (rest, elem) = parse_csi(input).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        elem,
        AnsiElement::Csi {
            params: vec![38, 2, 255, 0, 0],
            intermediates: vec![],
            ignore: false
        }
    );
}

#[test]
fn test_osc_title() {
    let input = b"\x1b]0;Nueva Terminal\x07";
    let (rest, elem) = parse_osc(input).unwrap();
    assert!(rest.is_empty());
    assert_eq!(
        elem,
        AnsiElement::Osc(vec![b"0".to_vec(), b"Nueva Terminal".to_vec()])
    );
}

#[test]
fn test_invalid_sequence() {
    let input = b"\x1b[?12";
    let err = parse_csi(input).unwrap_err();
    assert!(matches!(err, nom::Err::Incomplete(_)));
}

#[cfg(test)]
mod tests {
    mod html_integration_tests;
}
