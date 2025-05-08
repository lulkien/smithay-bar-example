use smithay_client_toolkit::{
    delegate_layer,
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
};
use wayland_client::{Connection, QueueHandle, protocol::wl_shm::Format};

use super::SimBar;

delegate_layer!(SimBar);

impl LayerShellHandler for SimBar {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        println!("closed layer");
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        _configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|monitor| &monitor.layer_surface == layer)
        {
            let surface = monitor.layer_surface.wl_surface().clone();

            if monitor.buffer.is_none() {
                println!("Create buffer and make init draw call");

                let stride = monitor.draw_size.width * 4;

                let (buffer, _) = monitor
                    .pool
                    .create_buffer(
                        monitor.draw_size.width as i32,
                        monitor.draw_size.height as i32,
                        stride as i32,
                        Format::Argb8888,
                    )
                    .expect("Failed to create new buffer.");

                buffer
                    .attach_to(monitor.layer_surface.wl_surface())
                    .expect("Failed to attach buffer");

                monitor.buffer = Some(buffer);

                self.draw(qh, &surface);
            }
        }
    }
}
