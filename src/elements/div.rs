use std::collections::HashMap;
use std::io::{Result, Error};

use crate::elements::element::*;
use crate::elements::style::{Display, Style, StyleKind};
use crate::utils::WhitespaceChecker;

pub struct Div {
    id: Option<&'static str>,
    class: Option<&'static str>,
    style: HashMap<StyleKind, Style>,
}

impl Element for Div {
    fn set_id(&mut self, id: &'static str) -> Result<()> {
        Ok(())
    }

    fn set_class(&mut self, class: &'static str) -> Result<()> {
        Ok(())
    }

    fn set_style(&mut self, style: Style) -> Result<()> {
        Ok(())
    }

    fn get_id(&self) -> Option<&'static str> {
        Some("")
    }

    fn get_class(&self) -> Option<&'static str> {
        Some("")
    }

    fn get_style(&self, kind: StyleKind) -> Option<&Style> {
        self.style.get(&kind)
    }

    fn set_attribute(&mut self, attribute: Attribute) -> Result<()> {
        self.get_style(StyleKind::DISPLAY);
        Ok(())
    }

    fn get_attribute(&self, attribute: &Attribute) -> Result<Attribute> {
        todo!()
    }
}

