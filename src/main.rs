mod data;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::calloop::Interest;
use smithay::reexports::calloop::Mode;
use smithay::reexports::calloop::PostAction;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::DisplayHandle;
use smithay::reexports::x11rb::protocol::xproto::Cursor;
use smithay::utils::Clock as SmithayClock;
use smithay::utils::Monotonic;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::shm::ShmState;
use smithay::wayland::socket::ListeningSocketSource;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::input::SeatState;
use smithay::desktop::Space;
use smithay::desktop::Window;
use smithay::wayland::data_device::DataDeviceState;
use smithay::input::Seat;
use smithay::input::pointer::CursorImageStatus;

fn main() -> anyhow::Result<(), anyhow::Error> {
    let event_loop: EventLoop<data::View> = EventLoop::try_new()?;
    let mut display: Display<data::State> = Display::new()?;
    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();
    println!("{:?}", socket_name);
    let eh = event_loop.handle();
    eh.insert_source(
        socket, // app wanting to connect
        |stream, _, data| {
            data.display
                .handle()
                .insert_client(stream, Arc::new(data::ClientData::default()))
                .unwrap();
        },
    )?;

    eh.insert_source(
        Generic::new(
            display.backend().poll_fd().as_raw_fd(), // convert the requests into raw fd (like a mailbox)
            Interest::READ, // Only activate if there is data wanting to be **read**
            Mode::Level,    // Stay on even if some of the data has been read
        ),
        |_, _, data| {
            data.display.dispatch_clients(&mut data.state).unwrap(); // handle the clients request
            Ok(PostAction::Continue)
        },
    )?;

    // Now we will make the data to put into the data::State struct
    
    let dh: DisplayHandle = display.handle();

    let time = SmithayClock::<Monotonic>::new().expect("FAIL to initialize clock");
    let compositor_state: CompositorState = CompositorState::new::<data::State>(&dh);
    let shm_state = ShmState::new::<data::State>(&dh, vec![]);
    let xdg_shell_state = XdgShellState::new::<data::State>(&dh);
    let cursor_status = CursorImageStatus::Default;
    let pointer_location = (0.0, 0.0).into();
    let space = Space::<Window>::default();
    let data_device_state = DataDeviceState::new::<data::State>(&dh);
    let output_manager_state = OutputManagerState::new_with_xdg_output::<data::State>(&dh);
    let mut seat_state = SeatState::<data::State>::new();
    let mut seat: Seat<data::State> = seat_state.new_wl_seat(&dh, "Noctura_seat");
    seat.add_keyboard(Default::default(), 300, 300)?;
    seat.add_pointer();

    let state: data::State = data::State {
        time,
        compositor_state,
        data_device_state,
        seat_state,
        shm_state,
        space,
        cursor_status,
        pointer_location,
        output_manager_state,
        xdg_shell_state
    };
    println!("Finished code");
    Ok(())
}