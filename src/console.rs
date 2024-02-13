use crate::{io::uart::Uart, mutex::Mutex};
use core::fmt::{self, Write};

static UART: Mutex<Option<Uart>> = Mutex::new(None);

pub fn initialize() {
    let mut uart = Uart::new();
    uart.initialize();

    let mut uart_mut = UART.lock();
    *uart_mut = Some(uart);
}

struct UartWriter;

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(uart) = *UART.lock() {
            for c in s.chars() {
                if c == '\n' {
                    uart.write(b'\r')
                }

                uart.write(c as u8);
            }
        }

        // We don't want to return an error if the UART hasn't been initialized yet,
        // as that may cause a panic, which would end up being useless.
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
