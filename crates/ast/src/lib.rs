#[derive(PartialEq, PartialOrd, Debug)]
pub struct Component {
    pub script: Script,
    pub style: Style,
    pub view: View,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Script {
    pub statements: Vec<ScriptStatement>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum ScriptStatement {}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Style {
    pub rules: Vec<StyleRule>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum StyleRule {}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct View {
    pub children: Vec<ViewNode>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum ViewNode {
    Text {
        data: String,
    },
    Element {
        name: String,
        attributes: Vec<ViewNodeElementAttribute>,
        children: Vec<ViewNode>,
    },
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ViewNodeElementAttribute {
    pub name: String,
    pub value: (),
}
