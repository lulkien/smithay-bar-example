use crate::simbar::DrawSize;

use super::{Component, Widgets};

#[allow(unused)]
pub struct CenterWidgets {
    pub components: Vec<Box<dyn Component + Send + Sync>>,
    pub height: u32,
}

#[allow(unused)]
impl Widgets for CenterWidgets {
    fn render(&self, area: DrawSize) -> Vec<Option<u32>> {
        let mut buffer = vec![None; (area.width * area.height) as usize];

        if self.components.is_empty() {
            return buffer;
        }

        let mut rendered = Vec::new();
        let mut total_width = 0;
        let mut max_height = 0;

        for component in self.components.iter() {
            let (bbox, pixels) = component.render();
            total_width += bbox.width;
            max_height = max_height.max(bbox.height);
            rendered.push((bbox, pixels));
        }

        if 0 == max_height {
            return buffer;
        }

        // Cap max_height by widget's height field
        max_height = max_height.min(self.height);

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
