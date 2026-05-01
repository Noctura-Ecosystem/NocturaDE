use smithay::backend::renderer::utils::on_commit_buffer_handler;
use smithay::desktop::Space;
use smithay::desktop::Window;
use smithay::input::Seat;
use smithay::input::SeatHandler;
use smithay::input::SeatState;
use smithay::input::pointer::CursorImageStatus;
use smithay::reexports::wayland_server::Client;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::backend;
use smithay::reexports::wayland_server::protocol::wl_buffer;
use smithay::reexports::wayland_server::protocol::wl_seat;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Serial;
use smithay::utils::{Clock, Logical, Monotonic, Point};
use smithay::wayland::buffer::BufferHandler;
use smithay::wayland::compositor::CompositorClientState;
use smithay::wayland::compositor::CompositorHandler;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::data_device::DataDeviceState;
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::shell::xdg::PopupSurface;
use smithay::wayland::shell::xdg::PositionerState;
use smithay::wayland::shell::xdg::ToplevelSurface;
use smithay::wayland::shell::xdg::XdgShellHandler;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::shm::ShmHandler;
use smithay::wayland::shm::ShmState;
use smithay::wayland::data_device::DataDeviceHandler;
use smithay::wayland::data_device::ClientDndGrabHandler;
use smithay::wayland::data_device::ServerDndGrabHandler;


pub struct Data {
    pub display: Display<State>,
    pub state: State,
    pub seat: Seat<State>,
}
pub enum Event {
    Spawn(String),
}

#[derive(Default)]
pub struct ClientData {
    pub compositor_state: CompositorClientState,
}

impl backend::ClientData for ClientData {}

pub struct State {
    pub time: Clock<Monotonic>,
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub seat_state: SeatState<Self>,
    pub shm_state: ShmState,
    pub space: Space<Window>,
    pub cursor_status: CursorImageStatus,
    pub pointer_location: Point<f64, Logical>,
    pub output_manager_state: OutputManagerState,
    pub xdg_shell_state: XdgShellState,
}

impl BufferHandler for State {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}
impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        // for new apps wanting a surface (window)
        &mut self.compositor_state
    }
    fn commit(&mut self, surface: &WlSurface) {
        // when an app requests to draw onto its surface
        on_commit_buffer_handler::<Self>(surface);
    }
    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        // for smithay managing identification
        let data: &ClientData = client.get_data::<ClientData>().unwrap();
        &data.compositor_state
    }
}
smithay::delegate_compositor!(State);

impl ShmHandler for State {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}
smithay::delegate_shm!(State);

impl SeatHandler for State {
    type KeyboardFocus = Window;
    type PointerFocus = Window;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        // if smithay wants to check the seat_state
        &mut self.seat_state
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, status: CursorImageStatus) {
        // if cursor icon changes (like from loading -> idle -> text)
        self.cursor_status = status;
    }

    fn focus_changed(
        &mut self,
        _seat: &Seat<Self>,
        _focused: std::option::Option<&smithay::desktop::Window>,
    ) { // when focused window changes
        // TODO: add window focus (could use tauri special commands)
    }
}
smithay::delegate_seat!(State);

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        // when smithay wants xdg_shell_state
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        // when new apps want to open
        let window: Window = Window::new(surface);
        self.space.map_element(window, (0, 0), true); // TODO: spawn element at the center of the screen
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) { // when a app wants to spawn a popup ie. context menus
        // TODO: make the popup logic
    }
    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}
}
smithay::delegate_xdg_shell!(State);

impl ClientDndGrabHandler for State {}
impl ServerDndGrabHandler for State {}

impl DataDeviceHandler for State {
    type SelectionUserData = ();

    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}
smithay::delegate_data_device!(State);

smithay::delegate_output!(State);
