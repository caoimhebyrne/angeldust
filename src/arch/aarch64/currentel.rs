use core::arch::asm;

/// Holds the current exception level.
///
/// https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CurrentEL--Current-Exception-Level?lang=en
#[derive(Clone, Copy, Debug)]
pub struct CurrentELRegister {
    /// Either 0, 1, 2 or 3.
    pub exception_level: u32,
}

impl CurrentELRegister {
    pub fn read() -> CurrentELRegister {
        let data: u32;
        unsafe {
            asm!("mrs {0:x}, CurrentEL", out(reg) data);
        }

        // https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CurrentEL--Current-Exception-Level?lang=en#fieldset_0-3_2
        let exception_level = data >> 2;
        CurrentELRegister { exception_level }
    }
}
