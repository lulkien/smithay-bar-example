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

use crate::{
    components::{CenterWidgets, Label, Padding, RgbColor, Widgets},
    configuration::THEME_CONFIG,
};

/// Represents the dimensions of a drawable surface in pixels.
///
/// This struct is used to store the width and height of a surface, such as a monitor's
/// rendering area in the `SimBar` Wayland client. It supports conversion from a tuple
/// for convenient initialization.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DrawSize {
    /// The width of the surface in pixels.
    pub width: u32,
    /// The height of the surface in pixels.
    pub height: u32,
}

impl From<(u32, u32)> for DrawSize {
    /// Converts a tuple of `(width, height)` into a `DrawSize`.
    ///
    /// # Arguments
    ///
    /// * `value` - A tuple where the first element is the width and the second is the height.
    ///
    /// # Example
    ///
    /// ```
    /// let size: DrawSize = (1920, 32).into();
    /// assert_eq!(size.width, 1920);
    /// assert_eq!(size.height, 32);
    /// ```
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

/// Represents a monitor in the `SimBar` Wayland client, managing its Wayland surface and buffer.
///
/// A `Monitor` corresponds to a single output (display) and handles its layer surface, shared
/// memory pool, and rendering buffer. It tracks whether it is the primary monitor for rendering
/// the status bar.
pub struct Monitor {
    /// The Wayland output (display) associated with this monitor.
    pub output: WlOutput,
    /// The layer surface for rendering the status bar on this monitor.
    pub layer_surface: LayerSurface,
    /// The shared memory pool for allocating buffers.
    pub pool: SlotPool,
    /// The current buffer for rendering, if configured.
    ///
    /// If `None`, the monitor requires reconfiguration (e.g., creating a new buffer).
    pub buffer: Option<Buffer>,
    /// The dimensions of the drawable area in pixels.
    pub draw_size: DrawSize,
    /// Whether this monitor is the primary one for rendering the status bar.
    pub is_primary: bool,
}

/// The main state of the `SimBar` Wayland client, managing monitors and Wayland protocols.
///
/// `SimBar` orchestrates the Wayland client’s interaction with the compositor, handling
/// multiple monitors, input events, and rendering of the status bar. It maintains the
/// state of Wayland protocols and tracks the last draw time for frame rate control.
pub struct SimBar {
    /// The registry state for discovering Wayland globals.
    pub registry_state: RegistryState,
    /// The seat state for handling input devices (e.g., pointer).
    pub seat_state: SeatState,
    /// The output state for managing monitors (displays).
    pub output_state: OutputState,
    /// The shared memory state for buffer allocation.
    pub shm: Shm,
    /// The compositor state for creating and managing surfaces.
    pub compositor: CompositorState,
    /// The layer shell state for creating layer surfaces (e.g., status bar).
    pub layer_shell: LayerShell,
    /// The list of monitors managed by the client.
    pub monitors: Vec<Monitor>,
    /// The optional pointer device for handling mouse events.
    pub pointer: Option<WlPointer>,
    /// Whether the client should exit.
    pub exit: bool,
    /// The timestamp of the last rendered frame, used for frame rate capping.
    pub last_draw_time: Instant,
}

impl SimBar {
    fn blend_pixels(fg: u32, bg: u32) -> u32 {
        let fg_a = (fg >> 24) & 0xFF; // Foreground alpha (0-255)
        let fg_r = (fg >> 16) & 0xFF;
        let fg_g = (fg >> 8) & 0xFF;
        let fg_b = fg & 0xFF;

        let bg_r = (bg >> 16) & 0xFF;
        let bg_g = (bg >> 8) & 0xFF;
        let bg_b = bg & 0xFF;

        let alpha = fg_a as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;

        let r = (fg_r as f32 * alpha + bg_r as f32 * inv_alpha).round() as u32;
        let g = (fg_g as f32 * alpha + bg_g as f32 * inv_alpha).round() as u32;
        let b = (fg_b as f32 * alpha + bg_b as f32 * inv_alpha).round() as u32;

        (0xFF << 24) | (r << 16) | (g << 8) | b
    }

    pub fn draw(&mut self, qh: &QueueHandle<Self>, surface: &WlSurface) {
        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|monitor| monitor.layer_surface.wl_surface() == surface)
        {
            let buffer: &mut Buffer = monitor.buffer.as_mut().expect("Buffer should be created");

            let canvas: &mut [u8] = monitor.pool.raw_data_mut(&buffer.slot());

            let hello: Label = Label {
                text: "Hello".to_owned(),
                fg_color: RgbColor::new(0xFF, 0xFF, 0xFF),
                bg_color: None,
                font_size: 25,
            };
            let pad = Padding(20);
            let world: Label = Label {
                text: "World".to_owned(),
                fg_color: RgbColor::new(0xFF, 0x00, 0xFF),
                bg_color: None,
                font_size: 25,
            };

            let center = CenterWidgets {
                components: vec![Box::new(hello), Box::new(pad), Box::new(world)],
                height: monitor.draw_size.height,
            };

            let data = center.render(monitor.draw_size);

            let bg_color: u32 = THEME_CONFIG.background_color.into();

            canvas
                .chunks_exact_mut(4)
                .enumerate()
                .for_each(|(i, chunk)| {
                    let pixel = if i < data.len() {
                        match data[i] {
                            Some(fg_pixel) => Self::blend_pixels(fg_pixel, bg_color),
                            None => bg_color,
                        }
                    } else {
                        bg_color
                    };

                    let array: &mut [u8; 4] = chunk.try_into().unwrap();
                    *array = pixel.to_le_bytes();
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

            monitor.layer_surface.commit();
        }
    }
}
