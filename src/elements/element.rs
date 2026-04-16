use std::io::Result;
use crate::elements::style::{Style, StyleKind};

pub trait Element {
    fn set_id(&mut self, id: &'static str) -> Result<()>;
    fn set_class(&mut self, class: &'static str) -> Result<()>;
    fn set_style(&mut self, style: Style) -> Result<()>;

    fn get_id(&self) -> Option<&'static str>;
    fn get_class(&self) -> Option<&'static str>;
    fn get_style(&self, kind: StyleKind) -> Option<&Style>;

    fn set_attribute(&mut self, attribute: Attribute) -> Result<()>;
    fn get_attribute(&self, attribute: &Attribute) -> Result<Attribute>;
}

#[derive(Debug)]
pub enum Attribute{
    ID(Option<&'static str>),
    CLASS(Option<&'static str>),
    STYLE(Vec<Style>),
}
