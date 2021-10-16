use html_lexer::{Attribute, HtmlLexer, Token};

#[test]
fn test_basic_tag() {
    let mut lexer = HtmlLexer::new("<h1></h1>");

    assert_eq!(
        Token::Tag {
            attributes: Vec::new(),
            is_end_tag: false,
            self_closing: false,
            tag_name: String::from("h1")
        },
        lexer.next(),
    );

    assert_eq!(
        Token::Tag {
            attributes: Vec::new(),
            is_end_tag: true,
            self_closing: false,
            tag_name: String::from("h1")
        },
        lexer.next(),
    );
}

#[test]
fn test_children_tag() {
    let mut lexer = HtmlLexer::new("<h1><div /></h1>");

    assert_eq!(
        Token::Tag {
            attributes: Vec::new(),
            is_end_tag: false,
            self_closing: false,
            tag_name: String::from("h1")
        },
        lexer.next(),
    );

    assert_eq!(
        Token::Tag {
            attributes: Vec::new(),
            is_end_tag: false,
            self_closing: true,
            tag_name: String::from("div")
        },
        lexer.next(),
    );

    assert_eq!(
        Token::Tag {
            attributes: Vec::new(),
            is_end_tag: true,
            self_closing: false,
            tag_name: String::from("h1")
        },
        lexer.next(),
    );
}

#[test]
fn test_attribute() {
    let mut lexer = HtmlLexer::new("<div backgroundColor=\"red\"></div>");

    assert_eq!(
        Token::Tag {
            attributes: vec![Attribute::from_name_value(
                "backgroundColor".into(),
                "red".into()
            )],
            is_end_tag: false,
            self_closing: false,
            tag_name: String::from("div")
        },
        lexer.next(),
    );
}
