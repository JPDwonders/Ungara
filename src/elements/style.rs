#[derive(Clone)]
pub enum Style {
    DISPLAY(Display),
    WIDTH(usize),
    HEIGHT(usize)
}

#[derive(Clone)]
pub enum Display {
    INLINE,
    BLOCK,
    FLEX,
    GRID,
    NONE
}