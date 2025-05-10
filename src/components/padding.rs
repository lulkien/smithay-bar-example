use super::{ArgbColor, BoundingBox, Component};

#[allow(unused)]
pub struct Padding(pub u32);

impl Component for Padding {
    fn render(&self) -> (BoundingBox, Vec<Option<ArgbColor>>) {
        (
            BoundingBox {
                width: self.0,
                height: 0,
            },
            vec![None; self.0 as usize],
        )
    }
}
