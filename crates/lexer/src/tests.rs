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
        (";", Token::Semicolon),
        (":", Token::Colon),
        ("::", Token::ColonColon),
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
        ("a    ", Token::Identifier("a".into())),
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
    let tests = vec![
        ("let", Token::Let),
        ("fn", Token::Fn),
        ("const", Token::Const),
        ("component", Token::Component),
    ];

    for (source, token) in tests {
        let lexer = Lexer::new(source);
        assert_eq!(lexer.token, token);
    }
}

#[test]
fn test_contextual() {
    let source = "let a = b + c;";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.token, Token::Let);
    let tokens = vec![
        Token::Identifier("a".into()),
        Token::Equal,
        Token::Identifier("b".into()),
        Token::Plus,
        Token::Identifier("c".into()),
        Token::Semicolon,
    ];
    for token in tokens {
        lexer.next();
        assert_eq!(lexer.token, token);
    }
}
