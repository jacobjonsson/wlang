use html_parser::HtmlParser;

#[test]
fn test_text() {
    let parser = HtmlParser::new("abc");
    let json = serde_json::to_string_pretty(&parser.parse()).unwrap();
    println!("{}", json);
    panic!("Trigger print");
}

#[test]
fn test_element() {
    let parser = HtmlParser::new("<div></div>");
    let json = serde_json::to_string_pretty(&parser.parse()).unwrap();
    println!("{}", json);
    panic!("Trigger print");
}

#[test]
fn test_nested_element() {
    let parser = HtmlParser::new("<div><div></div></div>");
    let json = serde_json::to_string_pretty(&parser.parse()).unwrap();
    println!("{}", json);
    panic!("Trigger print");
}
