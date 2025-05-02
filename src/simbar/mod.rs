mod monitor;

use monitor::Monitor;

use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_layer, delegate_output, delegate_pointer, delegate_registry,
    delegate_seat, delegate_shm,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::Capability,
    seat::{
        SeatHandler, SeatState,
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
    },
    shell::{
        WaylandSurface,
        wlr_layer::{
            Anchor, Layer, LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure,
        },
    },
    shm::{Shm, ShmHandler, slot::SlotPool},
};
use wayland_client::{
    Connection, QueueHandle,
    protocol::{
        wl_output::{Transform, WlOutput},
        wl_pointer::WlPointer,
        wl_seat::WlSeat,
        wl_shm::Format,
        wl_surface::WlSurface,
    },
};

use crate::configuration::SIMBAR_CONFIG;

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
}

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
        self.draw(qh, surface);
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

impl OutputHandler for SimBar {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _conn: &Connection, qh: &QueueHandle<Self>, output: WlOutput) {
        println!("new_output");

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

            let is_primary = SIMBAR_CONFIG
                .primary_output
                .map_or(info.logical_position == Some((0, 0)), |name| {
                    info.name.is_some_and(|monitor_name| name == monitor_name)
                });

            layer_surface.set_anchor(Anchor::TOP | Anchor::LEFT | Anchor::RIGHT);
            layer_surface.set_size(width, height);
            layer_surface.set_exclusive_zone(height as i32);
            layer_surface.commit();

            let pool = SlotPool::new((width * height * 4) as usize, &self.shm)
                .expect("Failed to create pool");

            self.monitors.push(Monitor {
                output,
                layer_surface,
                pool,
                draw_size: (width, height).into(),
                configured: false,
                is_primary,
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

                let width = SIMBAR_CONFIG.width.unwrap_or(output_width);
                let height = SIMBAR_CONFIG.height;

                monitor.layer_surface.set_size(width, height);
                monitor.layer_surface.set_exclusive_zone(height as i32);
                monitor.pool = SlotPool::new((width * height * 4) as usize, &self.shm)
                    .expect("Failed to create pool");
                monitor.layer_surface.commit();

                monitor.draw_size = (width, height).into();

                // Updated output -> need to be reconfig
                monitor.configured = false;
            }
        }
    }

    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, output: WlOutput) {
        println!("output_destroyed");
        self.monitors.retain(|monitor| monitor.output != output);
    }
}

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

impl SeatHandler for SimBar {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: WlSeat) {
        println!("new_seat");
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: WlSeat,
        capability: Capability,
    ) {
        println!("new_capability");
        if capability == Capability::Pointer && self.pointer.is_none() {
            println!("Set pointer capability");
            let pointer = self
                .seat_state
                .get_pointer(qh, &seat)
                .expect("Failed to create pointer");
            self.pointer = Some(pointer);
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: WlSeat,
        capability: Capability,
    ) {
        println!("remove_capability");
        if capability == Capability::Pointer && self.pointer.is_some() {
            self.pointer.take().unwrap().release();
        }
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _seat: WlSeat) {
        println!("remove_seat");
    }
}

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

impl ShmHandler for SimBar {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl SimBar {
    pub fn draw(&mut self, qh: &QueueHandle<Self>, surface: &WlSurface) {
        if let Some(monitor) = self
            .monitors
            .iter_mut()
            .find(|monitor| monitor.layer_surface.wl_surface() == surface)
        {
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

delegate_compositor!(SimBar);
delegate_output!(SimBar);
delegate_shm!(SimBar);
delegate_seat!(SimBar);
delegate_pointer!(SimBar);
delegate_layer!(SimBar);
delegate_registry!(SimBar);

impl ProvidesRegistryState for SimBar {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
