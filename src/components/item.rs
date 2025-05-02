use smithay_client_toolkit::shm::slot::SlotPool;

use super::Drawable;

#[allow(dead_code)]
pub struct Item {
    pub width: u32,
    pub height: u32,
    pub border_radius: u32,
    pub color: u32,
}

impl Drawable for Item {
    fn draw(
        &self,
        _pool: &mut SlotPool,
        _canvas: &mut [u8],
        _stride: i32,
        _buffer_width: i32,
        _buffer_height: i32,
    ) {
        todo!()
    }
}
