#[derive(PartialEq, PartialOrd, Debug)]
pub enum Token {
    Illegal,
    EOF,

    Semicolon,

    Identifier(String),
    Integer(String),

    Assign,
    Plus,
    Minus,
    HashBang,
    Asterisk,
    Slash,

    // Keywords
    Let,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Semicolon => ";".into(),
            Token::Let => "let".into(),
            Token::Identifier(i) => i.clone(),
            Token::Integer(i) => i.into(),
            Token::Assign => "=".into(),
            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::HashBang => "!".into(),
            Token::Asterisk => "*".into(),
            Token::Slash => "/".into(),
            Token::EOF => "EOF".into(),
            Token::Illegal => "Illegal".into(),
        }
    }
}

impl Token {
    pub fn from_str(value: &str) -> Token {
        Token::from(String::from(value))
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "let" => Token::Let,
            _ => Token::Identifier(value),
        }
    }
}
