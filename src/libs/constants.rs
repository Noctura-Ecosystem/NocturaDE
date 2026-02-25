#![allow(warnings)]

use crate::libs::utils::log;

use std::ffi::OsString;
use crate::init_wayland;

// Wayland Server imports
use wayland_server::DisplayHandle;
use wayland_server::protocol::wl_surface::WlSurface;

// Smithay Core & Desktop
use smithay::desktop::{PopupManager, Space, Window as WLwindow};
use smithay::reexports::calloop::{EventLoop, LoopSignal};
use smithay::reexports::wayland_server::{Display, protocol::wl_shm};

// Smithay Wayland Protocols & States
use smithay::wayland::compositor::CompositorState as WlCompositorState;
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::shm::{ShmHandler, ShmState};
use smithay::wayland::socket::ListeningSocketSource;

// Selection / Data Device
use smithay::wayland::selection::data_device::{DataDeviceHandler, DataDeviceState};
use smithay::wayland::selection::SelectionHandler;

// Input
use smithay::input::{Seat, SeatState, SeatHandler};
use smithay::input::pointer::CursorImageStatus;

// Macros
use smithay::{delegate_compositor, delegate_data_device, delegate_output, delegate_seat, delegate_shm, delegate_xdg_shell};

use smithay::wayland::compositor::CompositorHandler;
use smithay::wayland::output::OutputHandler;

use smithay::wayland::selection::data_device::ClientDndGrabHandler;
use smithay::wayland::selection::data_device::ServerDndGrabHandler;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::desktop::space::render_output;


#[derive(Default)]
pub struct ClientState {
    pub compositor_state: smithay::wayland::compositor::CompositorClientState,
}

impl wayland_server::backend::ClientData for ClientState {}


enum CursorState {
    IDLE,
    MOVING,
    RESIZING,
}

enum WindowState {
    FULLSCREEN,
    MINIMIZE,
    NORMAL,
}

pub struct CompositorState {
    pub start_time: std::time::Instant,
    pub display_handle: DisplayHandle,
    pub socket: OsString,
    pub compositor: WlCompositorState,
    pub data_device: DataDeviceState,
    pub xdg_shell: XdgShellState,
    pub shm_state: ShmState,
    pub loop_signal: LoopSignal,
    pub space: Space<WLwindow>,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<CompositorState>,
    pub seat: Seat<CompositorState>,
    pub cursor_state: CursorState,
    pub windows: Vec<Window>,
    pub surfaces: Vec<WlSurface>,
}

pub struct Window {
    mapped: bool,
    grabbed: bool,
    resize: bool,
    size: (u32, u32),  // classical x,y cordinates followed throughout the script
    condition: WindowState,
    id: u32,
    name: String,
}




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

    }
    fn xdg_shell_state(&mut self) -> &mut smithay::wayland::shell::xdg::XdgShellState {
        &mut self.xdg_shell

    }
    fn new_toplevel(&mut self, surface: smithay::wayland::shell::xdg::ToplevelSurface) {
        println!("A new window (window) is being created!");
    }
    fn new_popup(&mut self, _surface: smithay::wayland::shell::xdg::PopupSurface, _positioner: smithay::wayland::shell::xdg::PositionerState) {
        println!("A new window (popup) is being created!");
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
    }
}

/* DELEGATE */
smithay::delegate_xdg_shell!(CompositorState);
smithay::delegate_compositor!(CompositorState);
smithay::delegate_output!(CompositorState);
smithay::delegate_seat!(CompositorState);
smithay::delegate_shm!(CompositorState);
smithay::delegate_data_device!(CompositorState);



impl CompositorState {
    pub fn new<'a>(display: &'a mut Display<Self>, event_loop: &mut EventLoop<'a, Self>) -> Self {

        /// INITIALIZE ATTRIBUTES ///

        let display_handle = display.handle();

        let xdg_shell_state = XdgShellState::new::<Self>(&display_handle);
        let compositor_state = WlCompositorState::new::<Self>(&display_handle);
        let popups = PopupManager::default();
        let shm_state = ShmState::new::<Self>(&display_handle, vec![]);

        let data_device_state = DataDeviceState::new::<Self>(&display_handle);
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&display_handle);

        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&display_handle, "winit");
        seat.add_keyboard(Default::default(), 200, 25);
        seat.add_pointer();
        let loop_signal = event_loop.get_signal();
        let listening_socket = ListeningSocketSource::new_auto().unwrap();
        let socket_name = listening_socket.socket_name().to_os_string();
        println!("{:?}", socket_name);
        let space = Space::default();
        let cursor_state: CursorState = CursorState::IDLE;
        let start_time = std::time::Instant::now();
        

        Self {
            start_time,
            display_handle,
            socket: socket_name,
            compositor: compositor_state,
            data_device: data_device_state,
            xdg_shell: xdg_shell_state,
            shm_state,
            loop_signal,
            space,
            output_manager_state,
            seat_state,
            seat,
            cursor_state,
            windows: Vec::new(),
            surfaces: Vec::new(),
        }

    }

    pub fn prep(&mut self, event_loop: &mut EventLoop::<Self>) {
        init_wayland(event_loop, self);
    }

    pub fn add_window(&mut self, x: u32, y: u32, id: u32, name: String) {
        let mut new_win = Window::new(x , y, id, name);
        self.windows.push(new_win);
    }

    pub fn render_frame(&mut self, renderer: &mut GlesRenderer, output: &smithay::output::Output) {
    }
}


impl Window {
    pub fn new(x: u32, y: u32, id: u32, name: String) -> Self {
        Self {
            mapped: false,
            grabbed: false,
            resize: false,
            size: (x, y),  // classical x,y cordinates followed throughout the script
            condition: WindowState::NORMAL,
            id: id,
            name: name,
        }
    }
}

/*

        let (mut backend, winit) = winit::init()?;
        let mode = Mode {
            size: backend.window_size(),
            refresh: 60_000,
        };
*/


/*

4 | use utils::log;
  |     ^^^^^ help: a similar path exists: `smithay::utils`
5 | use basic_functions::init_wayland;
  |     ^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `basic_functions`
*/

