#![no_std]
#![no_main]

global_asm!(include_str!("boot/boot.S"));

mod arch;
mod console;
mod cpu;
mod io;
mod mutex;

use crate::{
    arch::aarch64::currentel::CurrentELRegister,
    cpu::{raspberry_pi, RaspberryPi},
    io::{framebuffer, mailbox},
};
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
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

    // If we are not on Exception Level 1, we need to bail out, something has gone wrong.
    let el_register = CurrentELRegister::read();
    println!(
        "[angeldust::init] running in exception level {}",
        el_register.exception_level
    );

    if el_register.exception_level != 1 {
        panic!(
            "expected to be in exception level 1, but is in level {}",
            el_register.exception_level
        );
    }

    // After we verify that this board is supported, initialize the global mailbox.
    mailbox::initialize();

    // Once the mailbox is ready, we can initialize the framebuffer.
    framebuffer::initialize();

    framebuffer::instance()
        .fill_area(50, 50, 1280 - 50, 720 - 50, 0xFF_FF0000)
        .expect("framebuffer::fill_area() failed");

    panic!("reached end of init()");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);

    loop {
        unsafe { asm!("wfe") }
    }
}
