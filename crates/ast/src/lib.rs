#![allow(dead_code)]

pub struct Component {
    script: Script,
    style: Style,
    view: View,
}

pub struct Script {}

pub struct Style {}

pub struct View {
    children: Vec<ViewNode>,
}

pub enum ViewNode {
    Text {
        data: String,
    },
    Element {
        name: String,
        attributes: Vec<ViewNodeElementAttribute>,
    },
}

pub struct ViewNodeElementAttribute {
    name: String,
    value: (),
}
