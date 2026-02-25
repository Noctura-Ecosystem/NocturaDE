mod libs;

use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use libs::constants::CompositorState;
use crate::libs::basic_functions::init_wayland;

fn main() {
    let mut display = Display::<CompositorState>::new().expect("Failed to create Wayland display");

    let mut event_loop: EventLoop<CompositorState> = EventLoop::try_new().expect("Failed to create event loop");
    let mut state = CompositorState::new(&mut display, &mut event_loop);
    state.prep(&mut event_loop);
    let mut display_handle = state.display_handle.clone();

    event_loop.run(None, &mut state, |state| {
        display_handle.flush_clients().unwrap();
        state.space.refresh();
    }).unwrap();
}