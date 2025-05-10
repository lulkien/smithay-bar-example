use crate::{components::Component, simbar::DrawSize};

use super::{ArgbColor, Widgets};

#[allow(unused)]
pub struct LeftWidgets {
    pub components: Vec<Box<dyn Component + Send + Sync>>,
    pub height: u32,
}

#[allow(unused)]
impl Widgets for LeftWidgets {
    fn render(&self, area: DrawSize) -> Vec<Option<ArgbColor>> {
        todo!()
    }
}
