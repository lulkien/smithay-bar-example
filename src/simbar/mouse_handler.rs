use smithay_client_toolkit::{
    delegate_pointer,
    seat::pointer::{PointerEvent, PointerEventKind, PointerHandler},
    shell::WaylandSurface,
};
use wayland_client::{Connection, QueueHandle, protocol::wl_pointer::WlPointer};

use super::SimBar;

delegate_pointer!(SimBar);

impl PointerHandler for SimBar {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            if self
                .monitors
                .iter()
                .any(|monitor| &event.surface == monitor.layer_surface.wl_surface())
            {
                if let PointerEventKind::Press { .. } = event.kind {
                    self.exit = true; // Exit on any button press
                }
            }
        }
    }
}
