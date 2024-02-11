use bitflags::{bitflags, Flags};
use core::ptr::{read_volatile, write_volatile};

use crate::cpu::RaspberryPi;

#[derive(Clone, Copy, Debug)]
pub struct Uart {
    registers: Registers,
}

// 11.5: Register View
// https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A149%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C139.396%2Cnull%5D
#[derive(Clone, Copy, Debug)]
struct Registers {
    data: *mut u32,
    flag: *mut u32,
    line_control: *mut u32,
    control: *mut u32,
}

bitflags! {
    /// 11.5. Register View - FR Register
    /// https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#reg-UART-FR
    struct Flag: u32 {
        const TransmitFIFOFull = 1 << 5;
        const ReceiveFIFOFUll = 1 << 6;
    }

    // 11.5. Register View - LCRH Register
    // https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#reg-UART-LCRH
    struct LineControlFlags: u32 {
        const EnableFIFO = 1 << 4;
        const DisableFIFO = 0 << 4;
        const WordLengthEight = 0b11 << 5;
    }

    // 11.5. Register View - CR Register
    // https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#reg-UART-CR
    struct ControlFlags: u32 {
        const UARTEnable = 1 << 0;
        const TransmitEnable = 1 << 8;
        const ReceiveEnable = 1 << 9;
    }
}

impl Uart {
    /// Creates a new instance of [Uart].
    ///
    /// # Safety
    /// - This function assumes that the [RaspberryPi::peripheral_base_address] is valid.
    pub fn new() -> Uart {
        let base_address = unsafe {
            RaspberryPi::instance()
                .peripheral_base_address()
                .byte_offset(0x201000) as *mut u32
        };
        Uart {
            registers: unsafe { Registers::new(base_address) },
        }
    }

    /// Initializes this [Uart] by disabling it, setting our desired options, and re-enabling it.
    ///
    /// This does not have to be called before writing anything, but is most-likely required
    /// on actual hardware to prevent bugs with the output.
    ///
    /// https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#reg-UART-CR
    pub fn initialize(&mut self) {
        // 1. Disable the UART
        Registers::write_value(self.registers.control, 0);

        // 2. Wait for the end of transmision, or reception of the current character.
        // no-op

        // 3a. Flush the transmit FIFO by setting the FEN bit to 0 in the LCRH register.
        Registers::write_bits(self.registers.line_control, LineControlFlags::DisableFIFO);

        // 3b. Reprogram the line control register
        Registers::write_bits(
            self.registers.line_control,
            LineControlFlags::EnableFIFO | LineControlFlags::WordLengthEight,
        );

        // 4 + 5. Reprogram the control register + Enable the UART
        Registers::write_bits(
            self.registers.control,
            ControlFlags::UARTEnable | ControlFlags::ReceiveEnable | ControlFlags::TransmitEnable,
        );
    }

    pub fn write(&self, byte: u8) {
        // Loop until the UART is clear again.
        while Registers::read_register::<_, Flag>(self.registers.flag)
            .contains(Flag::TransmitFIFOFull)
        {}

        Registers::write_value(self.registers.data, byte.into());
    }
}

impl Registers {
    /// Creates a new instance of [Registers].
    ///
    /// # Safety
    /// - This assumes that the provided [uart_base] is valid.
    pub const unsafe fn new(uart_base: *mut u32) -> Registers {
        Registers {
            data: uart_base,
            flag: uart_base.byte_offset(0x18),
            line_control: uart_base.byte_offset(0x2C),
            control: uart_base.byte_offset(0x30),
        }
    }

    /// Writes a value to the register at [register].
    ///
    /// # Safety
    /// - This function assumes that [register] is valid.
    pub(crate) fn write_value<T>(register: *mut T, value: T) {
        unsafe { write_volatile(register, value) }
    }

    /// Writes a bit flag to the register at [register].
    ///
    /// # Safety
    /// - This function assumes that [register] is valid.
    pub(crate) fn write_bits<I, F>(register: *mut I, flag: F)
    where
        F: Flags<Bits = I>,
    {
        Registers::write_value(register, flag.bits())
    }

    /// Reads the bits from the register at [register].
    /// Calls [Flags::from_bits] to parse the bits into a readable format.
    ///
    /// # Safety
    /// - This function assumes that [register] is valid.
    pub(crate) fn read_register<I, F>(register: *mut I) -> F
    where
        F: Flags<Bits = I>,
    {
        let value = unsafe { read_volatile(register) };
        F::from_bits_retain(value)
    }
}

/// # Safety
/// - We always use [Uart] within a [crate::Mutex].
unsafe impl Send for Uart {}

/// # Safety
/// - We always use [Uart] within a [crate::Mutex].
unsafe impl Sync for Uart {}
