// 1. THIS MUST BE THE ABSOLUTE FIRST LINE
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod vga_buffer;
pub mod gdt;         // GDT + TSS setup — MUST be loaded before the IDT
pub mod interrupts;  // IDT with exception handlers

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 2. Clear the old text and print the new boot sequence
    println!("NyxOS is waking up...");

    // 3. Load the GDT+TSS first — the CPU needs a valid TSS to handle interrupts
    gdt::init();
    println!("GDT and TSS loaded.");
    
    // 4. NOW load the IDT (the exception handler table)
    interrupts::init_idt();
    println!("Interrupt Descriptor Table loaded.");

    // 5. NOW we trigger the breakpoint, safely caught by the IDT
    x86_64::instructions::interrupts::int3();

    println!("It didn't crash! The CPU returned execution back to the OS!");

    // 6. The infinite loop must be at the very bottom, OUTSIDE of everything else
    loop {
        x86_64::instructions::hlt(); // Halt the CPU until the next interrupt
    }
}