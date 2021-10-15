#[derive(PartialEq, PartialOrd, Debug)]
pub struct Html {
    pub children: Vec<HtmlNode>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum HtmlNode {
    Element(HtmlElement),
    Text(HtmlText),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct HtmlElement {
    pub tag_name: String,
    pub children: Vec<HtmlNode>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct HtmlText {
    pub value: String,
}
