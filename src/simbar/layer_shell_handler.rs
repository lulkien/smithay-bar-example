use smithay_client_toolkit::{
    delegate_layer,
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
};
use wayland_client::{Connection, QueueHandle};

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
            println!("configure layer");

            let surface = monitor.layer_surface.wl_surface().clone();

            if !monitor.configured {
                println!("Init draw");
                monitor.configured = true;
                self.draw(qh, &surface);
            }
        }
    }
}
