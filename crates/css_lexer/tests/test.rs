use css_lexer::{Lexer, TokenKind};
use cursor::StringCursor;

fn assert_kind(source: &str, kind: TokenKind) {
    let cursor = StringCursor::new(source);
    let mut lexer = Lexer::new(cursor);
    assert_eq!(lexer.next().unwrap().kind, kind);
}

#[test]
fn test_hash() {
    let tests = [
        ("#", TokenKind::Delim { value: '#' }),
        (
            "#abc",
            TokenKind::Hash {
                value: "abc".into(),
                is_id: true,
            },
        ),
        (
            "#123",
            TokenKind::Hash {
                value: "123".into(),
                is_id: false,
            },
        ),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}
#[test]
fn test_string() {
    let tests = [
        (
            "\"abc\"",
            TokenKind::String {
                value: "abc".into(),
            },
        ),
        (
            "'abc'",
            TokenKind::String {
                value: "abc".into(),
            },
        ),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}

#[test]
fn test_at_keyword() {
    let tests = [
        (
            "@media",
            TokenKind::AtKeyword {
                value: "media".into(),
            },
        ),
        (
            "@keyframe",
            TokenKind::AtKeyword {
                value: "keyframe".into(),
            },
        ),
        ("@", TokenKind::Delim { value: '@' }),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}

#[test]
fn test_tokens() {
    let tests = [
        ("", TokenKind::EOF),
        ("(", TokenKind::OpenParen),
        (")", TokenKind::CloseParen),
        ("{", TokenKind::OpenBrace),
        ("}", TokenKind::CloseBrace),
        ("[", TokenKind::OpenBracket),
        ("]", TokenKind::CloseBracket),
        (";", TokenKind::Semicolon),
        (",", TokenKind::Comma),
        (":", TokenKind::Colon),
        ("<!--", TokenKind::CDO),
        ("-->", TokenKind::CDC),
        ("<", TokenKind::Delim { value: '<' }),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}

#[test]
fn test_name_like() {
    let tests = [
        (
            "url(http)",
            TokenKind::Url {
                value: "http".into(),
            },
        ),
        (
            "url(\"http\")",
            TokenKind::Function {
                value: "url".into(),
            },
        ),
        (
            "calc(",
            TokenKind::Function {
                value: "calc".into(),
            },
        ),
        (
            "abc",
            TokenKind::Ident {
                value: "abc".into(),
            },
        ),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}

#[test]
fn test_numbers() {
    let tests = [
        (
            "123",
            TokenKind::Number {
                value: "123".into(),
            },
        ),
        (
            "123.2",
            TokenKind::Number {
                value: "123.2".into(),
            },
        ),
        (
            "+123.2",
            TokenKind::Number {
                value: "+123.2".into(),
            },
        ),
        (
            "+123e5",
            TokenKind::Number {
                value: "+123e5".into(),
            },
        ),
        (
            "-123.2",
            TokenKind::Number {
                value: "-123.2".into(),
            },
        ),
        (
            "123px",
            TokenKind::Dimension {
                value: "123".into(),
                unit: "px".into(),
            },
        ),
        (
            "123%",
            TokenKind::Percent {
                value: "123".into(),
            },
        ),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}

#[test]
fn test_comments() {
    let tests = [
        ("/*       */", TokenKind::EOF),
        ("/*       */(", TokenKind::OpenParen),
        ("/*       *//", TokenKind::Delim { value: '/' }),
    ];

    for (source, kind) in tests {
        assert_kind(source, kind);
    }
}
