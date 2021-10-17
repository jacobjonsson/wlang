use html_parser::HtmlParser;
use insta::assert_json_snapshot;

#[test]
fn basic_test_cases() {
    assert_json_snapshot!(HtmlParser::new("abc").parse());
    assert_json_snapshot!(HtmlParser::new("<div />").parse());
    assert_json_snapshot!(HtmlParser::new("<div></div>").parse());
    assert_json_snapshot!(HtmlParser::new("<div><div><h1></h1></div></div>").parse());
    assert_json_snapshot!(HtmlParser::new("<div><div></div></div><div><div></div></div>").parse());
    assert_json_snapshot!(HtmlParser::new(
        "
        <div>
            <h1>Hello world</h1>
        </div>
    "
    )
    .parse());

    assert_json_snapshot!(
        HtmlParser::new("<div><link href=\"https://www.google.com\" /></div>").parse()
    );
}
