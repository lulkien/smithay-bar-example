use std::time::{Duration, Instant};

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
        _time: u32,
    ) {
        let now = Instant::now();

        let min_frame_time = Duration::from_nanos(1_000_000_000 / SIMBAR_CONFIG.frame_rate);

        if now.duration_since(self.last_draw_time) >= min_frame_time {
            self.draw(qh, surface);
            self.last_draw_time = now;
            return;
        }

        if let Some(monitor) = self
            .monitors
            .iter()
            .find(|monitor| monitor.layer_surface.wl_surface() == surface)
        {
            monitor
                .layer_surface
                .wl_surface()
                .frame(qh, monitor.layer_surface.wl_surface().clone());

            // monitor.layer_surface.commit();
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
