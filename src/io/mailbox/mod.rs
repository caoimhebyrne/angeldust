pub mod implementation;
pub mod message;
pub mod types;

pub use implementation::*;
pub use message::*;

use crate::mutex::Mutex;

pub static MAILBOX: Mutex<Option<Mailbox>> = Mutex::new(None);

pub fn initialize() {
    let mailbox = Mailbox::new();
    *MAILBOX.lock() = Some(mailbox);
}

pub fn instance() -> Mailbox {
    match *MAILBOX.lock() {
        Some(value) => value,
        _ => panic!("mailbox::initialize() should be called before mailbox::instance()"),
    }
}
