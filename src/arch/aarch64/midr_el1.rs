use core::arch::asm;

/// Stores identification information for the processor.
///
/// https://developer.arm.com/documentation/ddi0601/2023-03/AArch64-Registers/MIDR-EL1--Main-ID-Register?lang=en
#[derive(Clone, Copy, Debug)]
pub struct MainIdRegister {
    pub part_number: u32,
}

impl MainIdRegister {
    pub fn read() -> MainIdRegister {
        let data: u32;
        unsafe {
            asm!("mrs {0:x}, midr_el1", out(reg) data);
        }

        // https://developer.arm.com/documentation/ddi0601/2023-03/AArch64-Registers/MIDR-EL1--Main-ID-Register?lang=en#fieldset_0-15_4
        let part_number = (data >> 4) & 0xFFF;
        MainIdRegister { part_number }
    }
}
