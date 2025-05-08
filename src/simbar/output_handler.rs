use smithay_client_toolkit::{
    delegate_output,
    output::{OutputHandler, OutputState},
    shell::{
        WaylandSurface,
        wlr_layer::{Anchor, Layer},
    },
    shm::slot::{Buffer, SlotPool},
};
use wayland_client::{Connection, QueueHandle, protocol::wl_output::WlOutput};

use super::SimBar;
use crate::{configuration::SIMBAR_CONFIG, simbar::Monitor};

delegate_output!(SimBar);

impl OutputHandler for SimBar {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _conn: &Connection, qh: &QueueHandle<Self>, output: WlOutput) {
        if let Some(info) = self.output_state.info(&output) {
            println!(
                "Create surface for monitor: {}",
                info.clone().name.unwrap_or("Unknown".to_string())
            );
            // Create new wayland surface
            let surface = self.compositor.create_surface(qh);

            // Create new wayland layer for current output
            let layer_surface = self.layer_shell.create_layer_surface(
                qh,
                surface,
                Layer::Overlay,
                Some("sim_bar"),
                Some(&output),
            );

            let output_width = info
                .logical_size
                .map_or(SIMBAR_CONFIG.width_fallback, |(w, _)| w as u32);

            let width = SIMBAR_CONFIG.width.unwrap_or(output_width);
            let height = SIMBAR_CONFIG.height;
            let depth = 4;

            let is_primary = SIMBAR_CONFIG
                .primary_output
                .map_or(info.logical_position == Some((0, 0)), |name| {
                    info.name.is_some_and(|monitor_name| name == monitor_name)
                });

            layer_surface.set_anchor(Anchor::TOP | Anchor::LEFT | Anchor::RIGHT);
            layer_surface.set_size(width, height);
            layer_surface.set_exclusive_zone(height as i32);
            layer_surface.commit();

            let pool = SlotPool::new((width * height * depth) as usize, &self.shm)
                .expect("Failed to create pool");

            println!("Create new monitor: {width} x {height}");

            self.monitors.push(Monitor {
                output,
                layer_surface,
                pool,
                draw_size: (width, height).into(),
                is_primary,
                buffer: None,
            });
        }
    }

    fn update_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, output: WlOutput) {
        println!("update_output");

        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|monitor| monitor.output == output)
        {
            if let Some(info) = self.output_state.info(&output) {
                let output_width = info
                    .logical_size
                    .map_or(SIMBAR_CONFIG.width_fallback, |(w, _)| w as u32);

                let old_draw_size = monitor.draw_size;

                let width = SIMBAR_CONFIG.width.unwrap_or(output_width);
                let height = SIMBAR_CONFIG.height;

                monitor.layer_surface.set_size(width, height);
                monitor.layer_surface.set_exclusive_zone(height as i32);

                monitor.draw_size = (width, height).into();

                if old_draw_size != monitor.draw_size {
                    // Remove old buffer anyway
                    if monitor.buffer.is_some() {
                        let old_buffer: Buffer =
                            monitor.buffer.take().expect("Failed to take old buffer");

                        // Destroy old buffer
                        drop(old_buffer);
                    }

                    // Resize slot pool to match with new output size
                    monitor
                        .pool
                        .resize((width * height * 4) as usize)
                        .expect("Failed to create pool.");
                }

                monitor.layer_surface.commit();
            }
        }
    }

    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, output: WlOutput) {
        println!("output_destroyed");
        self.monitors.retain(|monitor| monitor.output != output);
    }
}
