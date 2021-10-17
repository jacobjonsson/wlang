use html_ast::{Element, Node, NodeRef};
use html_lexer::{HtmlLexer, Token};

pub struct HtmlParser {
    lexer: HtmlLexer,
    fragment: NodeRef,
    stack: Vec<NodeRef>,
    token: Token,
}

impl HtmlParser {
    pub fn new(source: &str) -> HtmlParser {
        let fragment = NodeRef::new(Node::Element(Element {
            name: "fragment".into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }));

        HtmlParser {
            lexer: HtmlLexer::new(source),
            stack: vec![fragment.clone()],
            fragment,
            token: Token::EOF,
        }
    }

    pub fn parse(mut self) -> NodeRef {
        self.next();
        while self.token != Token::EOF {
            self.handle_token();
        }

        return self.fragment;
    }

    fn current(&mut self) -> NodeRef {
        self.stack.last().unwrap().clone()
    }

    fn next(&mut self) -> &Token {
        self.token = self.lexer.next();
        &self.token
    }

    fn handle_token(&mut self) {
        match self.token {
            Token::Character(ch) if ch.is_whitespace() => {
                self.next();
            }
            Token::Character(_) => self.handle_text(),
            Token::Doctype => panic!("Does not support doctype"),
            Token::Tag { .. } => self.handle_tag(),
            Token::EOF => return,
        };
    }

    fn handle_tag(&mut self) {
        let parent = self.current();
        let name = self.token.tag_name();
        let mut element = Node::new_element(name.clone());
        for attribute in self.token.attributes() {
            element.set_element_attribute(attribute.name.clone(), attribute.value.clone());
        }

        if self.token.is_end_tag() {
            self.stack.pop();
            self.next();
            return;
        }

        let element_ref = NodeRef::new(element);

        if self.token.self_closing() {
            // We don't push it onto the stack if it's self closing.
            parent.borrow_mut().append_child(element_ref);
        } else {
            parent.borrow_mut().append_child(element_ref.clone());
            self.stack.push(element_ref.clone());
        }
        self.next();
    }

    fn handle_text(&mut self) {
        let mut text = Node::new_text();
        text.append_text(self.token.character());

        while let Token::Character(ch) = self.next() {
            text.append_text(*ch);
        }

        self.current().borrow_mut().append_child(NodeRef::new(text));
    }
}
