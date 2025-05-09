use crate::simbar::DrawSize;

mod label;
mod padding;

pub use label::Label;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    pub width: u32,
    pub height: u32,
}

#[allow(unused)]
pub trait Component {
    fn render(self) -> (BoundingBox, Vec<Option<u32>>);
}

#[allow(unused)]
pub trait Widgets {
    fn render(self, area: DrawSize) -> Vec<Option<u32>>;
}

#[allow(unused)]
pub struct LeftWidget<T: Component> {
    pub components: Vec<T>,
    pub height: u32,
}

#[allow(unused)]
pub struct CenterWidget<T: Component> {
    pub components: Vec<T>,
    pub height: u32,
}

#[allow(unused)]
pub struct RightWidget<T: Component> {
    pub components: Vec<T>,
    pub height: u32,
}

#[allow(unused)]
impl<T: Component> Widgets for LeftWidget<T> {
    fn render(self, area: DrawSize) -> Vec<Option<u32>> {
        todo!()
    }
}

#[allow(unused)]
impl<T: Component> Widgets for CenterWidget<T> {
    fn render(self, area: DrawSize) -> Vec<Option<u32>> {
        let mut buffer = vec![None; (area.width * area.height) as usize];

        if self.components.is_empty() {
            return buffer;
        }

        let mut rendered = Vec::new();
        let mut total_width = 0;
        let mut max_height = 0;

        for component in self.components {
            let (bbox, pixels) = component.render();
            total_width += bbox.width;
            max_height = max_height.max(bbox.height);
            rendered.push((bbox, pixels));
        }

        // Calculate offsets to center the big bounding box in the canvas
        let y_offset = if area.height > max_height {
            (area.height - max_height) / 2
        } else {
            (max_height - area.height) / 2
        };

        let x_offset = if area.width > total_width {
            (area.width - total_width) / 2
        } else {
            (total_width - area.width) / 2
        };

        // Copy each component's pixels directly to the canvas buffer
        let mut start_x: usize = 0;

        for (bbox, pixels) in rendered {
            let start_y: usize = if bbox.height < max_height {
                (max_height - bbox.height) / 2
            } else {
                0
            } as usize;

            // Copy pixels to canvas buffer
            for y in 0..bbox.height as usize {
                let canvas_y = y_offset as usize + start_y + y;
                if canvas_y >= area.height as usize {
                    continue;
                }

                for x in 0..bbox.width as usize {
                    let canvas_x = x_offset as usize + start_x + x;
                    if canvas_x >= area.width as usize {
                        continue;
                    }

                    let src_idx = y * bbox.width as usize + x;
                    let dest_idx = canvas_y * area.width as usize + canvas_x;

                    if src_idx < pixels.len() && dest_idx < buffer.len() {
                        buffer[dest_idx] = pixels[src_idx];
                    }
                }
            }

            start_x += bbox.width as usize;
        }

        buffer
    }
}

#[allow(unused)]
impl<T: Component> Widgets for RightWidget<T> {
    fn render(self, area: DrawSize) -> Vec<Option<u32>> {
        todo!()
    }
}
