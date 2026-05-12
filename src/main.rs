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
    // Initialize the VGA writer with yellow text on black background.
    // The buffer pointer is cast from physical address 0xb8000 (standard VGA text location).
    let mut writer = vga_buffer::Writer {
        column_position: 0,
        row_position: 0,
        color_code: vga_buffer::ColorCode::new(vga_buffer::Color::Yellow, vga_buffer::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga_buffer::Buffer) },
    };

    let os_name = "NyxOS";
    let version = 1;
    
    // Write boot messages using the VGA writer.
    // We can use both direct write_string() and formatted write!() macro.
    writer.write_string("Hello this is Nithwin\n ");
    write!(writer, "Welcome to {} Version {}!\n", os_name, version).unwrap();
    
    // Test hexadecimal formatting: 0xDEADBEEF is a common test value in software.
    write!(writer, "Booting from Bare Metal... Hex test: 0x{:X}\n", 0xDEADBEEF_u32).unwrap();

    // Infinite loop: keep the OS running.
    // In a real OS, this would dispatch interrupts, manage processes, etc.
    loop {}
}