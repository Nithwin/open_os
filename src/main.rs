#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// We use unsafe(no_mangle) to satisfy modern Rust requirements,
// ensuring the bootloader can find this exact function name.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Pointer to the hardware VGA buffer
    let vga_buffer = 0xb8000 as *mut u8;
    
    let text = b"System Online: Welcome to NyxOS!";

    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            // Write the character
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the color (0x0A is Light Green)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb0; 
        }
    }

    loop {}
}