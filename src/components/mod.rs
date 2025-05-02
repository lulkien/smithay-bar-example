use smithay_client_toolkit::shm::slot::SlotPool;

mod item;

#[allow(dead_code)]
pub trait Drawable {
    fn draw(
        &self,
        pool: &mut SlotPool,
        canvas: &mut [u8],
        stride: i32,
        buffer_width: i32,
        buffer_height: i32,
    );
}
