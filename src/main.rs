#![allow(warnings)]
mod libs;
use std::sync::Arc;
use crate::libs::constants::ClientState;
use smithay::wayland::socket::ListeningSocketSource;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use libs::constants::CompositorState;
use crate::libs::basic_functions::init_wayland;
use crate::libs::basic_functions::prep;
use calloop::generic::Generic;
use calloop::Interest;
use calloop::PostAction;
use smithay::reexports::calloop::Mode;

fn main() {
    let mut display = Display::<CompositorState>::new().expect("Failed to create Wayland display");
    let mut event_loop: EventLoop<CompositorState> = EventLoop::try_new().expect("Failed to create event loop");
    
    let mut state = CompositorState::new(display, &mut event_loop);


    init_wayland(&mut event_loop, &mut state);
    prep(&mut state, &mut event_loop);
    
    std::process::Command::new("weston-terminal").spawn().ok();

    event_loop.run(None, &mut state, |state| {
        state.display.flush_clients().unwrap();
        state.space.refresh();
}).unwrap();
}