mod center_widget;
mod left_widgets;
mod right_widgets;

use crate::{components::ArgbColor, simbar::DrawSize};
use std::sync::{OnceLock, RwLock};

pub use center_widget::CenterWidgets;
pub use left_widgets::LeftWidgets;
pub use right_widgets::RightWidgets;

#[allow(unused)]
pub trait Widgets {
    fn render(&self, area: DrawSize) -> Vec<Option<ArgbColor>>;
}

#[allow(unused)]
pub struct SimbarWidgets {
    left: OnceLock<RwLock<LeftWidgets>>,
    center: OnceLock<RwLock<CenterWidgets>>,
    right: OnceLock<RwLock<RightWidgets>>,
}
