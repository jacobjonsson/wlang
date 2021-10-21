use css_lexer_next::Lexer;
use css_lexer_next::Token;

fn assert_token(source: &str, token: Token) {
    assert_eq!(Lexer::new(source).next(), token);
}

#[test]
fn test_comments() {
    let tests = [
        ("/* */", Token::EndOfFile),
        ("/*      */", Token::EndOfFile),
        (
            "/*
            

        */",
            Token::EndOfFile,
        ),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_whitespace() {
    let tests = [("       ", Token::Whitespace), (" ", Token::Whitespace)];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_string() {
    let tests = [
        ("\"abc\"  ", Token::String("abc")),
        ("\"abc", Token::BadString("abc")),
        ("\"a'bc", Token::BadString("a'bc")),
        ("\"a\\\"bc", Token::BadString("a\\\"bc")),
        ("'abc'", Token::String("abc")),
        ("'abc", Token::BadString("abc")),
        ("'a\"bc", Token::BadString("a\"bc")),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_tokens() {
    let tests = [
        ("{", Token::OpenCurlyBracket),
        ("}", Token::CloseCurlyBracket),
        ("[", Token::OpenSquareBracket),
        ("]", Token::CloseSquareBracket),
        ("(", Token::OpenParenthesis),
        (")", Token::CloseParenthesis),
        (",", Token::Comma),
        (":", Token::Colon),
        (";", Token::Semicolon),
        ("/", Token::Delim('/')),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_hash() {
    let tests = [
        ("#", Token::Delim('#')),
        ("#abc", Token::IDHash("abc")),
        ("#a\\:b", Token::IDHash("a\\:b")),
        ("#123", Token::Hash("123")),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_multiple_tokens() {
    let tests = [(
        "#abc ",
        [Token::IDHash("abc"), Token::Whitespace, Token::EndOfFile],
    )];

    for (source, tokens) in tests {
        let mut lexer = Lexer::new(source);

        for token in tokens {
            assert_eq!(lexer.next(), token)
        }
    }
}

#[test]
fn test_numbers() {
    let tests = [
        ("+123", Token::Number("+123")),
        ("+123%", Token::Percentage("+123")),
        ("+123px", Token::Dimension("+123", "px")),
        ("-123", Token::Number("-123")),
        ("-123%", Token::Percentage("-123")),
        ("-123px", Token::Dimension("-123", "px")),
        (".123", Token::Number(".123")),
        (".123%", Token::Percentage(".123")),
        (".123px", Token::Dimension(".123", "px")),
        ("123.4", Token::Number("123.4")),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}

#[test]
fn test_ident() {
    let tests = [
        ("abc", Token::Ident("abc")),
        ("calc(", Token::Function("calc")),
        ("var(", Token::Function("var")),
    ];

    for (source, token) in tests {
        assert_token(source, token);
    }
}
