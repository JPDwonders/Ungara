use std::io::Result;

pub trait Element {
    fn set_attribute(&mut self, attribute: AttributeWithValue) -> Result<()>;
    fn get_attribute(&self, attribute: &Attribute) -> Result<AttributeWithValue>;
}

pub enum AttributeWithValue {
    ID(Option<String>),
    CLASS(Option<String>),
    STYLE(Option<crate::Style>),
}

pub enum Attribute {
    ID,
    CLASS,
    STYLE,
}
