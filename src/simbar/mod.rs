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
    shm::{
        Shm,
        slot::{Buffer, SlotPool},
    },
};
use wayland_client::{
    QueueHandle,
    protocol::{wl_output::WlOutput, wl_pointer::WlPointer, wl_surface::WlSurface},
};

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl DrawSize {
    pub fn area(self) -> u32 {
        self.height * self.width
    }
}

#[allow(dead_code)]
pub struct Monitor {
    pub output: WlOutput,
    pub layer_surface: LayerSurface,
    pub pool: SlotPool,
    pub buffer: Option<Buffer>, // if buffer is None, then the monitor must be re-configurate
    pub draw_size: DrawSize,
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
            let buffer = monitor.buffer.as_ref().expect("Buffer should be created");

            let _canvas = monitor.pool.raw_data_mut(
                &monitor
                    .buffer
                    .as_ref()
                    .expect("Buffer should be created.")
                    .slot(),
            );

            let _canvas = buffer.canvas(&mut monitor.pool).unwrap();

            let canvas = monitor
                .pool
                .canvas(
                    &monitor
                        .buffer
                        .as_ref()
                        .expect("Buffer should be created.")
                        .slot(),
                )
                .expect("Failed to accquire canvas");

            println!("Canvas length: {}", canvas.len());

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

            monitor.layer_surface.wl_surface().damage_buffer(
                0,
                0,
                monitor.draw_size.width as i32,
                monitor.draw_size.height as i32,
            );

            monitor
                .layer_surface
                .wl_surface()
                .frame(qh, monitor.layer_surface.wl_surface().clone());

            monitor
                .buffer
                .as_ref()
                .expect("Buffer should be created")
                .attach_to(monitor.layer_surface.wl_surface())
                .expect("Failed to attach buffer");

            monitor.layer_surface.commit();
        }
    }
}
