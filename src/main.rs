mod data;
use smithay::reexports::calloop::EventLoop;
fn main() -> anyhow::Result<(), anyhow::Error> {
    let mut event_loop: EventLoop<data::View> = EventLoop::try_new()?;
    Ok(())
}