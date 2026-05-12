#![no_std]
#![no_main]

//! NyxOS - A bare-metal OS kernel.
//!
//! This is the entry point for the kernel. It initializes the VGA text buffer
//! and prints boot messages. Since there's no standard library, we use `#![no_std]`
//! and define a custom panic handler.

pub mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

/// Panic handler: called when a panic occurs.
///
/// In a full OS, this would log the panic and halt gracefully.
/// For now, we loop forever (the CPU will eventually halt or continue spinning).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point.
///
/// This function is called by the bootloader after setting up basic CPU state.
/// The `#[unsafe(no_mangle)]` and `extern "C"` ensure the linker can find this
/// symbol and call it using C calling conventions.
///
/// # Safety
/// This function uses unsafe to cast a raw pointer to VGA text buffer memory
/// (0xb8000) into a mutable reference. This is safe only if:
/// - The OS is running on x86 in real mode or protected mode
/// - The physical address 0xb8000 is mapped to VGA memory
/// - Nothing else writes to this memory simultaneously
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let os_name = "NyxOS";
    let version = 1;
    
    // Write boot messages using the VGA writer.
    // We can use both direct write_string() and formatted write!() macro.
    println!("Hello this is Nithwin");
    print!("This is my custom print function");
    // Infinite loop: keep the OS running.
    // In a real OS, this would dispatch interrupts, manage processes, etc.
    loop {}
}