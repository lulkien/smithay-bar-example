use std::sync::{OnceLock, RwLock};

use crate::simbar::DrawSize;

mod center_widget;
mod colors;
mod label;
mod left_widgets;
mod padding;
mod right_widgets;

pub use center_widget::CenterWidgets;
#[allow(unused)]
pub use colors::{ArgbColor, RgbColor};
pub use label::Label;
pub use left_widgets::LeftWidgets;
pub use padding::Padding;
pub use right_widgets::RightWidgets;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
}

#[allow(unused)]
pub trait Component {
    fn render(&self) -> (BoundingBox, Vec<Option<u32>>);
}

#[allow(unused)]
pub trait Widgets {
    fn render(&self, area: DrawSize) -> Vec<Option<u32>>;
}

#[allow(unused)]
pub struct SimbarWidgets {
    left: OnceLock<RwLock<LeftWidgets>>,
    center: OnceLock<RwLock<CenterWidgets>>,
    right: OnceLock<RwLock<RightWidgets>>,
}
