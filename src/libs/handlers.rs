
impl OutputHandler for CompositorState {}
impl smithay::input::SeatHandler for CompositorState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, _status: CursorImageStatus) {}
}
impl ShmHandler for CompositorState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
impl smithay::wayland::buffer::BufferHandler for CompositorState {
    fn buffer_destroyed(&mut self, _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer) {}
}
impl DataDeviceHandler for CompositorState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device
    }
}
impl SelectionHandler for CompositorState {
    type SelectionUserData = ();
}
impl ClientDndGrabHandler for CompositorState {}
impl ServerDndGrabHandler for CompositorState {}

impl smithay::wayland::shell::xdg::XdgShellHandler for CompositorState {
    fn reposition_request(&mut self, _surface: smithay::wayland::shell::xdg::PopupSurface, _positioner: smithay::wayland::shell::xdg::PositionerState, _token: u32) {

        println!("A new window (toplevel) is being created!");
    }
    fn xdg_shell_state(&mut self) -> &mut smithay::wayland::shell::xdg::XdgShellState {

        &mut self.xdg_shell

    }
    fn new_toplevel(&mut self, _surface: smithay::wayland::shell::xdg::ToplevelSurface) {

        println!("A new window (toplevel) is being created!");
        println!("{:?}", self.surfaces);

    }
    fn new_popup(&mut self, _surface: smithay::wayland::shell::xdg::PopupSurface, _positioner: smithay::wayland::shell::xdg::PositionerState) {
        println!("A new window (toplevel) is being created!");
    }
    fn grab(&mut self, _surface: smithay::wayland::shell::xdg::PopupSurface, _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat, _serial: smithay::utils::Serial) {}
}


impl CompositorHandler for CompositorState {
    fn compositor_state(&mut self) -> &mut WlCompositorState {
        &mut self.compositor
    }

   fn client_compositor_state<'a>(&self, client: &'a smithay::reexports::wayland_server::Client) -> &'a smithay::wayland::compositor::CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, _surface: &WlSurface) {
        // Leave empty for now
    }
}
/* DELEGATE */
smithay::delegate_xdg_shell!(CompositorState);
smithay::delegate_compositor!(CompositorState);
smithay::delegate_output!(CompositorState);
smithay::delegate_seat!(CompositorState);
smithay::delegate_shm!(CompositorState);
smithay::delegate_data_device!(CompositorState);