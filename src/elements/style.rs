mirror_enum! {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Style,

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum StyleKind {
        DISPLAY(Display),
        WIDTH(usize),
        HEIGHT(usize),
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Display {
    INLINE,
    BLOCK,
    FLEX,
    GRID,
    NONE
}
