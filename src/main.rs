mod data;
use std::sync::Arc;
use smithay::reexports::calloop::EventLoop;
use smithay::wayland::socket::ListeningSocketSource;
use smithay::reexports::wayland_server::Display;
use smithay::reexports::calloop::Mode;
use std::os::fd::AsRawFd;
use smithay::reexports::calloop::Interest;
use smithay::reexports::calloop::PostAction;
use smithay::reexports::calloop::generic::Generic;


fn main() -> anyhow::Result<(), anyhow::Error> {
    let event_loop: EventLoop<data::View> = EventLoop::try_new()?;
    let mut display: Display<data::State> = Display::new()?;
    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();
    println!("{:?}", socket_name);
    let eh = event_loop.handle();
    eh.insert_source(
        socket, // app wanting to connect
        | stream, _, data | {
            data.display.handle()
                .insert_client(stream, Arc::new(data::ClientData::default()))
                .unwrap();
        }
    )?;

    eh.insert_source(
        Generic::new(
            display.backend().poll_fd().as_raw_fd(), // convert the requests into raw fd (like a mailbox)
            Interest::READ, // Only activate if there is data wanting to be **read**
            Mode::Level, // Stay on even if some of the data has been read
        ), | _, _, data | {
            data.display.dispatch_clients(&mut data.state).unwrap(); // handle the clients request
            Ok(PostAction::Continue)
        }
    )?;
    Ok(())
}