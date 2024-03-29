// Make sure the linker puts this at the start of the kernel image
.section ".text.boot"

// Execution starts here
.global _start

_start:
    // Store the processor ID in `x0`.
    mrs     x0, mpidr_el1
    and     x0, x0, #3
    cbz     x0, check_el_and_drop

    // Otherwise, halt the processor indefinately.
halt:  
    wfe
    b       halt

check_el_and_drop:
    // Store the current exception level in `x0`.
    mrs     x0, CurrentEL
    and     x0, x0, #0xC
    cmp     x0, #8

    // if (x0 & 0xC) == 8:
    b.eq    el2_to_el1

    // if (x0 & 0xC) > 8:
    b.gt    el3_to_el2

    // else:
    b el1_entry

el3_to_el2:
    // Initialize SCLTR_EL2 and HCR_EL2 before entering EL2
    msr sctlr_el2, xzr
    msr hcr_el2, xzr

    mrs x0, SCR_EL3
    orr x0, x0, #(0b1<<10) // 0b1 = The execution state for EL2 is AArch64.
    orr x0, x0, #(0b1<<0)  // 0b1 = EL1 is non-secure world.
    msr SCR_EL3, x0

    mov X0, #0b01001     // 0b01001 = EL2h
    msr SPSR_EL3, X0

    // Go to the `el2_to_el1` routine when in EL2 (after eret).
    adr x0, el2_to_el1
    msr elr_el3, x0

    eret

el2_to_el1:
    // Allow SIMD and floating point registers to be accessed in EL1.
    mov x0, #(0b11 << 20)     // 0b11 = This control does not cause execution of any instructions to be trapped.
    msr cpacr_el1, x0

    // Use aarch64 when executing in EL1.
    mov x0, #(0b1 << 31)      // 0b1 = The Execution state for EL1 is AArch64.
    msr hcr_el2, x0

    // Set the aarch64 exception level.
    mov x0, #(0b0101 << 0)    // 0b0101 = EL1h
    msr spsr_el2, x0

    // Go to the `entry` routine when in EL1 (after eret).
    ldr x0, =el1_entry
    msr elr_el2, x0

    eret

el1_entry:
    // We should be in EL1 now!
    // It doesn't really matter if we're not... our C code will complain pretty soon.

    // Set stack to start below our code.
    ldr     x0, =_start
    mov     sp, x0

    // Clean the BSS section.
    // This is where our uninitialized variables are stored.
clear_bss:
    // Store the start of the BSS in `x0`.
    ldr     x0, =__bss_start

    // Store the end/size of the BSS in `w2`.
    ldr     w2, =__bss_size

1:
    // If the end/size is zero, quit the loop.
    cbz     w2, run_init
    str     xzr, [x0], #8
    sub     w2, w2, #1

    // Continue the loop if `w2` is non-zero.
    cbnz    w2, 1b

run_init:
    // Jump to our init() function
    bl      init

    // If it does return, halt the master core too
    b       halt