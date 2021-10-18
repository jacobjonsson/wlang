use css_lexer::{Lexer, Token};

#[test]
fn test_whitespace() {
    assert_eq!(Lexer::new("   ").next(), Token::Whitespace);
}

#[test]
fn test_string() {
    let tests = [("'abc'", "abc"), ("\"abc\"", "abc")];

    for (source, expected) in tests {
        assert_eq!(Lexer::new(source).next(), Token::String(expected));
    }
}

#[test]
fn test_eof() {
    assert_eq!(Lexer::new("").next(), Token::EndOfFile);
}

#[test]
fn test_punctuator() {
    let tests = [
        ("(", Token::OpenParen),
        (")", Token::CloseParen),
        ("{", Token::OpenBrace),
        ("}", Token::CloseBrace),
        ("[", Token::OpenBracket),
        ("]", Token::CloseBracket),
        (",", Token::Comma),
        (":", Token::Colon),
        (";", Token::Semicolon),
        ("+", Token::Delim('+')),
        ("-", Token::Delim('-')),
        ("#", Token::Delim('#')),
        ("@", Token::Delim('@')),
    ];

    for (source, token) in tests {
        assert_eq!(Lexer::new(source).next(), token);
    }
}

#[test]
fn test_id_hash() {
    let tests = [
        (
            "#abc",
            Token::Hash {
                value: "abc",
                id: true,
            },
        ),
        (
            "#_abc",
            Token::Hash {
                value: "_abc",
                id: true,
            },
        ),
        (
            "#_abc123",
            Token::Hash {
                value: "_abc123",
                id: true,
            },
        ),
    ];

    for (source, token) in tests {
        assert_eq!(Lexer::new(source).next(), token);
    }
}

#[test]
fn test_numbers() {
    let tests = [
        (
            "+123",
            Token::Number {
                int_value: Some(123),
                value: 123.,
            },
        ),
        (
            "-123",
            Token::Number {
                int_value: Some(-123),
                value: -123.,
            },
        ),
        (
            "1.2",
            Token::Number {
                int_value: None,
                value: 1.2,
            },
        ),
        (
            "100",
            Token::Number {
                int_value: Some(100),
                value: 100.,
            },
        ),
        (
            "+122.2",
            Token::Number {
                int_value: None,
                value: 122.2,
            },
        ),
        (
            "-122.2",
            Token::Number {
                int_value: None,
                value: -122.2,
            },
        ),
        (
            ".1",
            Token::Number {
                int_value: None,
                value: 0.1,
            },
        ),
    ];

    for (source, token) in tests {
        assert_eq!(Lexer::new(source).next(), token);
    }
}

#[test]
fn test_dimension() {
    let tests = [
        (
            "123px",
            Token::Dimension {
                value: 123.,
                int_value: Some(123),
                unit: "px",
            },
        ),
        (
            "123.4em",
            Token::Dimension {
                value: 123.4,
                int_value: None,
                unit: "em",
            },
        ),
        (
            ".123em",
            Token::Dimension {
                value: 0.123,
                int_value: None,
                unit: "em",
            },
        ),
        ("100%", Token::Percentage { value: 1. }),
        ("30%", Token::Percentage { value: 0.3 }),
    ];

    for (source, token) in tests {
        assert_eq!(Lexer::new(source).next(), token);
    }
}
