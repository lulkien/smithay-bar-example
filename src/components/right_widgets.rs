use crate::simbar::DrawSize;

use super::{Component, Widgets};

#[allow(unused)]
pub struct RightWidgets {
    pub components: Vec<Box<dyn Component + Send + Sync>>,
    pub height: u32,
}

#[allow(unused)]
impl Widgets for RightWidgets {
    fn render(&self, area: DrawSize) -> Vec<Option<u32>> {
        todo!()
    }
}
