pub mod css;
pub mod html;
pub mod script;

use css::Css;
use html::Html;
use script::Script;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Component {
    html: Html,
    css: Css,
    script: Script,
}
