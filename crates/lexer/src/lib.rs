#[cfg(test)]
mod test;

use std::str::Chars;

use syntax::{SyntaxKind, Token};

#[derive(Debug)]
pub struct SyntaxError {
    kind: SyntaxErrorKind,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub enum SyntaxErrorKind {
    UnclosedStringLiteral,
}

/// Tokenize the entire source file into a vector
pub fn tokenize_file(source: &str) -> (Vec<Token>, Vec<SyntaxError>) {
    if source.is_empty() {
        Default::default()
    }

    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut offset = 0;

    for (token, error) in tokenize(source) {
        let token_len = token.len;
        let new_offset = offset + token_len;

        tokens.push(token);

        if let Some(error) = error {
            errors.push(SyntaxError {
                kind: error,
                start: offset,
                end: new_offset,
            });
        }

        offset = new_offset;
    }

    todo!()
}

/// Tokenize returns an iterator over the string
/// Calling each will return a token and an optional error message
pub fn tokenize(mut source: &str) -> impl Iterator<Item = (Token, Option<SyntaxErrorKind>)> + '_ {
    std::iter::from_fn(move || {
        if source.is_empty() {
            return None;
        }

        let (token, err) = lex(source);
        source = &source[token.len as usize..];
        Some((token, err))
    })
}

fn lex(source: &str) -> (Token, Option<SyntaxErrorKind>) {
    assert!(!source.is_empty());

    if source.starts_with(is_whitespace) {
        (
            Token {
                kind: SyntaxKind::Whitespace,
                len: source
                    .find(is_not_whitespace)
                    .unwrap_or_else(|| source.len()),
            },
            None,
        )
    } else if source.starts_with(is_digit) {
        (consume_integer_or_float(source), None)
    } else if source.starts_with(is_identifier) {
        (
            Token {
                kind: SyntaxKind::Identifier,
                len: source
                    .find(is_not_identifier)
                    .unwrap_or_else(|| source.len()),
            },
            None,
        )
    } else if source.starts_with('"') {
        consume_string(source)
    } else if source.starts_with('/') {
        consume_comment_or_slash(source)
    } else {
        let ch = source.chars().next().unwrap();
        (
            Token {
                kind: match ch {
                    '(' => SyntaxKind::LeftParen,
                    ')' => SyntaxKind::RightParen,
                    '{' => SyntaxKind::LeftBrace,
                    '}' => SyntaxKind::RightBrace,
                    '[' => SyntaxKind::LeftBracket,
                    ']' => SyntaxKind::RightBracket,
                    '+' => SyntaxKind::Plus,
                    '-' => SyntaxKind::Minus,
                    '*' => SyntaxKind::Star,
                    '%' => SyntaxKind::Percent,
                    _ => SyntaxKind::Error,
                },
                len: ch.len_utf8(),
            },
            None,
        )
    }
}

/// Either consumes a comment or a single slash
/// This function assumes that the leading slash has been verified.
fn consume_comment_or_slash(source: &str) -> (Token, Option<SyntaxErrorKind>) {
    let mut chars = source.chars();

    // If the second char in the iterator is not a slash then it's not a comment
    if chars.clone().nth(1) != Some('/') {
        return (
            Token {
                kind: SyntaxKind::Slash,
                // Is this always valid..?
                len: 1,
            },
            None,
        );
    }

    let initial_len = source.len();

    chars.next(); // The leading slash
    chars.next(); // The second slash

    // Keep looping until we hit a newline or eof
    loop {
        match chars.next() {
            None | Some('\n') => {
                return (
                    Token {
                        kind: SyntaxKind::Comment,
                        len: initial_len - chars.as_str().len(),
                    },
                    None,
                )
            }

            _ => {}
        }
    }
}

fn consume_integer_or_float(source: &str) -> Token {
    let mut chars = source.chars();
    let initial_len = source.len();

    // Eat all the decimal digits
    eat_decimal_digits(&mut chars);

    match (chars.clone().next(), chars.clone().nth(1)) {
        (Some('.'), Some('0'..='9')) => {
            // Eat the leading dot.
            chars.next();
            eat_decimal_digits(&mut chars);
            Token {
                kind: SyntaxKind::Float,
                len: initial_len - chars.as_str().len(),
            }
        }

        (Some('e' | 'E'), _) => todo!("Exponents are not supported yet"),

        _ => Token {
            kind: SyntaxKind::Integer,
            len: initial_len - chars.as_str().len(),
        },
    }
}

/// Consumes decimal digits
fn eat_decimal_digits<'a>(chars: &mut Chars<'a>) {
    loop {
        match chars.clone().next() {
            Some('_') => {
                chars.next();
            }
            Some('0'..='9') => {
                chars.next();
            }
            _ => break,
        }
    }
}

/// Eats a string until it reaches the end character
fn consume_string(source: &str) -> (Token, Option<SyntaxErrorKind>) {
    let mut chars = source.chars();
    let initial_len = source.len();

    // Eat the initial character
    chars.next();

    loop {
        match chars.next() {
            // An backslash escapes any subsequent character
            Some('\\') => {
                chars.next(); // The the subsequent char;
            }

            Some('"') => {
                return (
                    Token {
                        kind: SyntaxKind::String,
                        len: initial_len - chars.as_str().len(),
                    },
                    None,
                );
            }

            Some(_) => {}

            None => {
                return (
                    Token {
                        kind: SyntaxKind::String,
                        len: initial_len - chars.as_str().len(),
                    },
                    Some(SyntaxErrorKind::UnclosedStringLiteral),
                )
            }
        }
    }
}

fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

fn is_not_whitespace(ch: char) -> bool {
    !is_whitespace(ch)
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_identifier(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

fn is_not_identifier(ch: char) -> bool {
    !is_identifier(ch)
}
