use super::tokenize;
use super::SyntaxErrorKind;
use super::SyntaxKind;

macro_rules! test_valid_token {
    ($name:ident, $input:expr, $kind:expr, $len:expr) => {
        #[test]
        fn $name() {
            let (token, _) = tokenize($input).next().unwrap();
            assert_eq!(token.kind, $kind);
            assert_eq!(token.len, $len);
        }
    };
}

macro_rules! test_invalid_token {
    ($name:ident, $input:expr, $kind:expr, $len:expr, $err:expr) => {
        #[test]
        fn $name() {
            let (token, err) = tokenize($input).next().unwrap();
            assert_eq!(token.kind, $kind);
            assert_eq!(token.len, $len);
            assert_eq!(err.unwrap(), $err);
        }
    };
}

// Valid tokens
test_valid_token!(plus_token, "+", SyntaxKind::Plus, 1);
test_valid_token!(minus_token, "-", SyntaxKind::Minus, 1);
test_valid_token!(slash_token, "/", SyntaxKind::Slash, 1);
test_valid_token!(start_token, "*", SyntaxKind::Star, 1);
test_valid_token!(percent_token, "%", SyntaxKind::Percent, 1);
test_valid_token!(ident_token, "abc", SyntaxKind::Identifier, 3);
test_valid_token!(func_token, "func", SyntaxKind::Identifier, 4);
test_valid_token!(integer_token, "123", SyntaxKind::Integer, 3);
test_valid_token!(float_token, "123.123", SyntaxKind::Float, 7);
test_valid_token!(string_token, "\"abc\"", SyntaxKind::String, 5);
test_valid_token!(comment_token, "// a long comment", SyntaxKind::Comment, 17);

// Invalid tokens
test_invalid_token!(
    invalid_string_token,
    "\"abc",
    SyntaxKind::String,
    4,
    SyntaxErrorKind::UnclosedStringLiteral
);
