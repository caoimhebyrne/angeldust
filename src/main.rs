#![no_std]
#![no_main]

global_asm!(include_str!("boot/boot.S"));

mod console;
mod mailbox;
mod mutex;
mod uart;

use crate::mailbox::{test_mailbox, Mailbox};
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

// TODO: Read the MIDR_EL1 register to determine the board type.
//       https://developer.arm.com/documentation/ddi0601/2023-12/AArch64-Registers/MIDR-EL1--Main-ID-Register

// Raspberry Pi 4:
// const BASE_ADDRESS: u32 = 0xFE00_0000;

// Raspberry Pi 3:
const BASE_ADDRESS: *mut u8 = 0x3F00_0000 as *mut u8;

#[no_mangle]
pub extern "C" fn init() -> ! {
    // We must do this as early as possible in order to get information printed out to the Uart.
    console::initialize();

    println!("[angeldust::init] hello from rust!");

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
