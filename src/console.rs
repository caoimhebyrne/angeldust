use core::{
    fmt::{self, Write},
    ptr::write_volatile,
};

use crate::BASE_ADDRESS;

struct UartWriter;

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                write_volatile(BASE_ADDRESS.byte_offset(0x201000), c as u8);
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    write!(UartWriter, "{}", args).ok();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
