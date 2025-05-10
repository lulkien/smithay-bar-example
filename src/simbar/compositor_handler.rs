use smithay_client_toolkit::{
    compositor::CompositorHandler, delegate_compositor, shell::WaylandSurface,
};
use wayland_client::{
    Connection, QueueHandle,
    protocol::{
        wl_output::{Transform, WlOutput},
        wl_surface::WlSurface,
    },
};

use super::SimBar;
use crate::configuration::SIMBAR_CONFIG;

const MIN_FRAME_TIME_MS: u32 = 1_000 / SIMBAR_CONFIG.frame_rate;

delegate_compositor!(SimBar);

impl CompositorHandler for SimBar {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_factor: i32,
    ) {
        println!("scale_factor_changed");
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_transform: Transform,
    ) {
        println!("transform_changed");
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        surface: &WlSurface,
        time: u32,
    ) {
        surface.frame(qh, surface.clone());

        let allow_draw = self
            .monitors
            .iter()
            .find(|m| m.layer_surface.wl_surface() == surface)
            .map(|m| m.is_primary)
            .unwrap_or(false);

        if allow_draw && time.wrapping_sub(self.last_frame_time) >= MIN_FRAME_TIME_MS {
            self.draw(qh, surface);
            self.last_frame_time = time;
        }

        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|m| m.layer_surface.wl_surface() == surface)
        {
            monitor.layer_surface.commit();
        }
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
        println!("surface_enter");
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
        println!("surface_leave");
    }
}
