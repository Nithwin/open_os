// Telling os Not to use standard library and we're on our own
// We're doing this because we're building an os which was an bare metal has nothing
#![no_std]

// This tell not to start with main function rather we will tell the program where to start from
#![no_main]

use core::panic::PanicInfo;

// This function is called when the program crashes.
// Because we have no standard library, we have to define exactly what a crash does!
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}


// This is the actual entry point of our Operating System.
// `extern "C"` tells the compiler to use the standard C calling convention, 
// which is what the hardware bootloader expects.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {
        
    }
}