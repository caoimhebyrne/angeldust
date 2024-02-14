pub mod implementation;
pub use implementation::*;

mod message;

use crate::mailbox;
use crate::mutex::Mutex;

pub static FRAMEBUFFER: Mutex<Option<Framebuffer>> = Mutex::new(None);

pub fn initialize() {
    let mut framebuffer = Framebuffer::default();
    framebuffer
        .initialize(&mailbox::instance())
        .expect("failed to initialize framebuffer");

    *FRAMEBUFFER.lock() = Some(framebuffer);
}

pub fn instance() -> Framebuffer {
    match *FRAMEBUFFER.lock() {
        Some(instance) => instance,
        _ => panic!("framebuffer::initialize() should be called before framebuffer::instance()"),
    }
}
