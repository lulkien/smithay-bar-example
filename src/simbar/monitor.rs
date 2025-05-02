#[allow(unused)]
use smithay_client_toolkit::{
    output::OutputInfo, shell::wlr_layer::LayerSurface, shm::slot::SlotPool,
};
use wayland_client::protocol::wl_output::WlOutput;

pub struct DrawSize {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for DrawSize {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

pub struct Monitor {
    pub output: WlOutput,
    // pub info: OutputInfo,
    pub layer_surface: LayerSurface,
    pub pool: SlotPool,
    pub draw_size: DrawSize,
    pub configured: bool,
    pub is_primary: bool,
}
