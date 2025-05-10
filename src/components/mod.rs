mod colors;
mod label;
mod padding;

#[allow(unused)]
pub use colors::{ArgbColor, RgbColor};
pub use label::Label;
pub use padding::Padding;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
}

#[allow(unused)]
pub trait Component {
    fn render(&self) -> (BoundingBox, Vec<Option<ArgbColor>>);
}
