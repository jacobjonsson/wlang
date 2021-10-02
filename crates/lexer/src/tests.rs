use super::*;

#[test]
fn test_punctuation() {
    let tests = vec![
        ("=", Token::Equal),
        ("==", Token::EqualEqual),
        ("(", Token::OpenParen),
        (")", Token::CloseParen),
        ("{", Token::OpenBrace),
        ("}", Token::CloseBrace),
        ("[", Token::OpenBracket),
        ("]", Token::CloseBracket),
        ("+", Token::Plus),
        ("-", Token::Minus),
        ("/", Token::Slash),
        ("*", Token::Star),
        ("%", Token::Percent),
        ("&", Token::Ampersand),
        ("&&", Token::AmpersandAmpersand),
        ("|", Token::Bar),
        ("||", Token::BarBar),
        (">", Token::Greater),
        (">=", Token::GreaterEqual),
        ("<", Token::Less),
        ("<=", Token::LessEqual),
    ];

    for (source, token) in tests {
        let lexer = Lexer::new(source);
        assert_eq!(lexer.token, token);
    }
}

#[test]
fn test_identifier() {
    let tests = vec![
        ("a", Token::Identifier("a".into())),
        ("_a", Token::Identifier("_a".into())),
        ("$a", Token::Identifier("$a".into())),
    ];

    for (source, token) in tests {
        let lexer = Lexer::new(source);
        assert_eq!(lexer.token, token);
    }
}

#[test]
fn test_keyword() {
    let tests = vec![("let", Token::Let), ("fn", Token::Fn)];

    for (source, token) in tests {
        let lexer = Lexer::new(source);
        assert_eq!(lexer.token, token);
    }
}
