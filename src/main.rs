#![no_std]
#![no_main]

global_asm!(include_str!("boot/boot.S"));

mod arch;
mod console;
mod cpu;
mod io;
mod mutex;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use crate::{
    cpu::{raspberry_pi, RaspberryPi},
    io::{test_mailbox, Mailbox},
};

#[no_mangle]
pub extern "C" fn init() -> ! {
    // We need to use this to be able to detect the board type.
    // This allows us to infer the peripheral base address.
    raspberry_pi::initialize();

    // We must do this as early as possible in order to get information printed out to the Uart.
    console::initialize();

    let board_type = RaspberryPi::instance().board_type();
    if !RaspberryPi::instance().is_supported() {
        panic!("the board type {:?} is not supported", board_type);
    }

    println!("[angeldust::init] hello from rust!");
    println!(
        "[angeldust::init] raspberry pi board type: {:?}",
        board_type
    );

    let mailbox = unsafe { Mailbox::new() };
    test_mailbox(&mailbox);

    panic!("reached end of init()");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n[angeldust::panic] reached panic handler...\n{}", info);

    loop {
        unsafe { asm!("wfe") }
    }
}
