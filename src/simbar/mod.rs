mod compositor_handler;
mod layer_shell_handler;
mod mouse_handler;
mod output_handler;
mod registry_handler;
mod seat_handler;
mod shm_handler;

use std::time::Instant;

use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    registry::RegistryState,
    seat::SeatState,
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShell, LayerSurface},
    },
    shm::{Shm, slot::SlotPool},
};
use wayland_client::{
    QueueHandle,
    protocol::{wl_output::WlOutput, wl_pointer::WlPointer, wl_shm::Format, wl_surface::WlSurface},
};

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

#[allow(dead_code)]
pub struct Monitor {
    pub output: WlOutput,
    pub layer_surface: LayerSurface,
    pub pool: SlotPool,
    pub draw_size: DrawSize,
    pub configured: bool,
    pub is_primary: bool,
}

pub struct SimBar {
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub output_state: OutputState,
    pub shm: Shm,
    pub compositor: CompositorState,
    pub layer_shell: LayerShell,
    pub monitors: Vec<Monitor>,
    pub pointer: Option<WlPointer>,
    pub exit: bool,
    pub last_draw_time: Instant,
}

impl SimBar {
    pub fn draw(&mut self, qh: &QueueHandle<Self>, surface: &WlSurface) {
        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|monitor| monitor.layer_surface.wl_surface() == surface)
        {
            println!("Draw");
            let width = monitor.draw_size.width as i32;
            let height = monitor.draw_size.height as i32;
            let stride = width * 4;

            let (buffer, canvas) = monitor
                .pool
                .create_buffer(width, height, stride, Format::Argb8888)
                .expect("create buffer");

            // Draw a solid #aaaaaa rectangle
            canvas.chunks_exact_mut(4).for_each(|chunk| {
                let a: u32 = 0xFF;
                let r: u32 = 0xAA;
                let g: u32 = 0xAA;
                let b: u32 = 0xAA;
                let color = (a << 24) + (r << 16) + (g << 8) + b;
                let array: &mut [u8; 4] = chunk.try_into().unwrap();
                *array = color.to_le_bytes();
            });

            // Re-draw damaged part
            monitor
                .layer_surface
                .wl_surface()
                .damage_buffer(0, 0, width, height);

            // request next frame
            monitor
                .layer_surface
                .wl_surface()
                .frame(qh, monitor.layer_surface.wl_surface().clone());

            buffer
                .attach_to(monitor.layer_surface.wl_surface())
                .expect("buffer attach");

            monitor.layer_surface.commit();
        }
    }
}
