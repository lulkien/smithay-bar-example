mod components;
mod configuration;
mod layout;
mod simbar;
mod widgets;

use std::time::Instant;

use configuration::SIMBAR_CONFIG;
use simbar::SimBar;
use smithay_client_toolkit::{
    compositor::CompositorState, output::OutputState, registry::RegistryState, seat::SeatState,
    shell::wlr_layer::LayerShell, shm::Shm,
};
use wayland_client::{Connection, globals::registry_queue_init};

fn main() {
    env_logger::init();

    assert_ne!(SIMBAR_CONFIG.width_fallback, 0);
    assert_ne!(SIMBAR_CONFIG.width, Some(0));
    assert_ne!(SIMBAR_CONFIG.height, 0);

    let conn = Connection::connect_to_env().unwrap();

    let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();

    let qh = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).expect("wl_compositor is not available");

    let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");

    let shm = Shm::bind(&globals, &qh).expect("wl_shm is not available");

    let mut sim_bar = SimBar {
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        output_state: OutputState::new(&globals, &qh),
        shm,
        compositor,
        layer_shell,
        monitors: Vec::new(),
        pointer: None,
        exit: false,
        last_draw_time: Instant::now(),
    };

    loop {
        event_queue.blocking_dispatch(&mut sim_bar).unwrap();

        if sim_bar.exit {
            println!("exiting example");
            break;
        }
    }
}
