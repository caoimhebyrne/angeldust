#![no_std]
#![no_main]

global_asm!(include_str!("boot/boot.S"));

mod console;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

const BASE_ADDRESS: u32 = 0x3F00_0000;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);

    loop {
        unsafe { asm!("wfe") }
    }
}

#[no_mangle]
pub extern "C" fn init() -> ! {
    println!("Hello, World!");

    panic!("reached end of init()");
}
