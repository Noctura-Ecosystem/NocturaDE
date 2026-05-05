mod data;
mod input;
use smithay::reexports::calloop;
use smithay::desktop::space::render_output;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::calloop::Interest;
use smithay::reexports::calloop::Mode;
use smithay::reexports::calloop::PostAction;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::wayland_server::DisplayHandle;
use smithay::utils::Clock as SmithayClock;
use smithay::utils::Monotonic;
use smithay::utils::Rectangle;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::shm::ShmState;
use smithay::wayland::socket::ListeningSocketSource;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use std::time::Duration;
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::input::SeatState;
use smithay::desktop::Space;
use smithay::desktop::Window;
use smithay::input::Seat;
use smithay::input::pointer::CursorImageStatus;
use smithay::backend::winit;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::output::Mode as wlMode;
use smithay::output::PhysicalProperties as wlPhysicalProperties;
use smithay::output::Subpixel;
use smithay::output::Output as wlOutput;
use std::time as stdTime;
use smithay::reexports::calloop::timer::Timer;
use smithay::reexports::calloop::timer::TimeoutAction;
use smithay::backend::winit::WinitEvent;
use smithay::backend::input::InputEvent;
use smithay::wayland::selection::data_device::DataDeviceState;
use calloop::generic::FdWrapper;
use smithay::input::pointer::CursorIcon;
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::damage::OutputDamageTracker;
use std::process::Command;
use crate::input::handlePointerButton;
use crate::input::handlePointerAbsolute;
use crate::input::handlekeyboard;

fn main() -> anyhow::Result<(), anyhow::Error> {
    let _ = std::fs::remove_file("/run/user/1000/wayland-1");
    let _ = std::fs::remove_file("/run/user/1000/wayland-1.lock");
    let mut event_loop: EventLoop<data::Data> = EventLoop::try_new()?;
    let mut display: Display<data::State> = Display::new()?;
    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();
    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", &socket_name);
    }
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
            unsafe { FdWrapper::new(display.backend().poll_fd().as_raw_fd()) }, 
            Interest::READ, 
            Mode::Level,    
        ),
        |_, _, data| {
            data.display.dispatch_clients(&mut data.state).unwrap();
            Ok(PostAction::Continue)
        },
    )?;

    // Now we will make the data to put into the data::State struct
    
    let dh: DisplayHandle = display.handle();

    let time = SmithayClock::<Monotonic>::new();
    let compositor_state: CompositorState = CompositorState::new::<data::State>(&dh);
    let shm_state = ShmState::new::<data::State>(&dh, vec![]);
    let xdg_shell_state = XdgShellState::new::<data::State>(&dh);
    let cursor_status = CursorImageStatus::Named(CursorIcon::Default);
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

    let mut data: data::Data = data::Data{
        state,
        display,
        seat,
    };

    // set up the output screen
    // TODO: make this into a function
    println!("--- PROGRAM TEST ---1");// Temporary debug line


    let (mut backend, mut winit) = winit::init::<GlesRenderer>().unwrap();

    println!("--- PROGRAM TEST --- 2");
    let size = backend.window_size();
    let mode = wlMode {
        size,           // This must be Size<i32, Physical>
        refresh: 60_000, // This is in milli-hertz (60fps = 60,000)
    };

    let physical_properties = wlPhysicalProperties {
        size: (0, 0).into(),
        subpixel: Subpixel::Unknown,
        make: "Noctura".into(),
        model: "Winit".into(),
    };
    let output = wlOutput::new("winit".to_string(), physical_properties);
    output.create_global::<data::State>(&data.display.handle());
    output.set_preferred(mode);
    data.state.space.map_output(&output, (0, 0));

    
    let cmd_res = Command::new("weston-terminal")
        .env("WAYLAND_DISPLAY", &socket_name)
        .spawn();

    match cmd_res {
        Ok(_) => {
            println!("Loaded weston")
        }
        Err(e) => {
            println!("COULD NOT LOAD WESTON TERMINAL DUE TO {:?}", e)
        }
    };



    let start_time = stdTime::Instant::now();
    let timer = Timer::immediate();
    // TODO: insert a pointer
    let mut output_damage_tracker = OutputDamageTracker::from_output(&output);
    let signal = event_loop.get_signal();

    eh.insert_source(timer, move |_, _, data| {
        let display = &mut data.display;
        let state: &mut data::State = &mut data.state;
        let seat = &mut data.seat;
        winit.dispatch_new_events(|e | {
            match e {
                WinitEvent::Input(InputEvent::PointerButton { event }) => {
                    handlePointerButton(event, state, seat);
                }
                WinitEvent::Input(InputEvent::PointerMotionAbsolute { event }) => {
                    handlePointerAbsolute(event, state, seat);
                }
                WinitEvent::Input(InputEvent::Keyboard { event }) => {
                    handlekeyboard(event, seat, state);
                }
                WinitEvent::Redraw => {
                    let size = backend.window_size();
                    let damages = Rectangle::from_size(size);
                    {
                    let (renderer, mut framebuffer) = backend.bind().unwrap();
                    let _ = render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
                        &output, renderer, &mut framebuffer, 1.0, 0, [&state.space], &[], &mut output_damage_tracker, [
                            0.0,
                            0.0,
                            0.0,
                            1.0
                        ]
                    );
                    }

                    backend.submit(Some(&[damages])).unwrap();
                    state.space.elements().for_each(|window| {
                        window.send_frame(
                            &output,
                            start_time.elapsed(),
                            Some(Duration::ZERO),
                            |_, _| Some(output.clone()),
                        )
                    });
                    state.space.refresh();
                    display.handle().flush_clients().unwrap();
                    backend.window().request_redraw();
                }
                WinitEvent::CloseRequested => {
                    signal.stop();
                }

                _ => {}

            };
        });


        TimeoutAction::ToDuration(Duration::from_millis(16))
    }).expect("Failed to insert Winit events");

    event_loop.run(None, &mut data, |_| {})?;

    println!("Finished code");
    Ok(())
}