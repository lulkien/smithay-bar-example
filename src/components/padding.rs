use super::{BoundingBox, Component};

#[allow(unused)]
pub struct Padding(u32);

impl Component for Padding {
    fn render(self) -> (BoundingBox, Vec<Option<u32>>) {
        todo!()
    }
}
