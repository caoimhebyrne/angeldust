use crate::arch::aarch64::midr_el1::MainIdRegister;

static mut INSTANCE: Option<RaspberryPi> = None;

pub fn initialize() {
    unsafe {
        INSTANCE = Some(RaspberryPi::new());
    }
}

/// Represents information about this Raspberry Pi.
///
/// Some of this information may be inferred, and may not be 100% accurate
/// in non-standard configurations, but should be fine for most use-cases.
#[derive(Clone, Copy, Debug)]
pub struct RaspberryPi {
    board_type: BoardType,
}

/// Represents the different Rasberry Pi board types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoardType {
    Pi1,
    Pi2,
    Pi3,
    Pi4,
    Unknown(u32),
}

impl RaspberryPi {
    /// Retrieves the global instance of [RaspberryPi].
    /// You must call [initialize] before running this.
    pub fn instance() -> RaspberryPi {
        unsafe { INSTANCE }.expect("To be Some(T)")
    }

    /// Whether or not the [RaspberryPi::board_type] is supported by the kernel.
    /// At the moment, only the Pi3 and Pi4 are supported.
    pub const fn is_supported(&self) -> bool {
        match self.board_type() {
            BoardType::Pi3 | BoardType::Pi4 => true,
            _ => false,
        }
    }

    /// Returns the [BoardType] inferred from the `midr_el1` register.
    pub const fn board_type(&self) -> BoardType {
        self.board_type
    }

    /// Returns the inferred peripheral base address for this Raspberry Pi.
    ///
    /// Since only the Raspberry Pi 3 and 4 are supported right now, anything else
    /// will return the Pi 3's base address in order to try and get a working UART.
    pub const fn peripheral_base_address(&self) -> *mut u8 {
        let address: u32 = match self.board_type() {
            BoardType::Pi4 => 0xFE00_0000,

            // Let's just fall back to the Pi3 peripheral base address for any other value.
            _ => 0x3F00_0000,
        };

        address as *mut u8
    }

    /// Creates a new instance of [RaspberryPi].
    /// This should only be called once, as the data will not change.
    fn new() -> RaspberryPi {
        let midr = MainIdRegister::read();
        RaspberryPi {
            board_type: BoardType::from(midr.part_number),
        }
    }
}

impl BoardType {
    /// https://wiki.osdev.org/Detecting_Raspberry_Pi_Board
    pub const fn from(part_number: u32) -> BoardType {
        match part_number {
            0xB76 => BoardType::Pi1,
            0xC07 => BoardType::Pi2,
            0xD03 => BoardType::Pi3,
            0xD08 => BoardType::Pi4,
            _ => BoardType::Unknown(part_number),
        }
    }
}
