//! Interrupt and Exception Handling.
//!
//! The IDT (Interrupt Descriptor Table) tells the CPU what code to run when
//! exceptions or interrupts occur. We configure handlers for:
//!   - Breakpoint exceptions (for debugging)
//!   - Double faults (when another exception cannot be handled)

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    /// The Interrupt Descriptor Table (IDT) for the kernel.
    ///
    /// The IDT is a table that maps exception/interrupt numbers to handler functions.
    /// When the CPU encounters an exception, it looks up the entry in the IDT
    /// and jumps to the corresponding handler function.
    ///
    /// We configure:
    ///   - Breakpoint (#3): For debugging breakpoints
    ///   - Double fault (#8): For unrecoverable errors (runs on dedicated IST stack)
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        // Map the breakpoint exception to our handler function
        // When the CPU executes 'int 3', it will call breakpoint_handler
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        // Map the double fault exception to our handler
        // Set stack_index so it uses the dedicated IST stack from the TSS
        // This ensures we have a valid stack even if the kernel stack overflowed
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        
        idt
    };
}

/// Load the IDT into the CPU.
///
/// This tells the CPU to use our IDT for exception handling.
/// The CPU will look up entries in this table whenever an exception or interrupt occurs.
/// Must be called during kernel initialization.
pub fn init_idt() {
    IDT.load();
}

/// Handler for breakpoint exceptions (exception #3).
///
/// When the CPU executes 'int 3' or encounters a breakpoint,
/// it calls this function with a snapshot of the CPU state at that moment.
///
/// The InterruptStackFrame contains information about what the CPU was doing:
/// - rip: The instruction that caused the interrupt
/// - rsp: The stack pointer
/// - Flags, etc.
///
/// We write "BRKP" directly to VGA memory to signal that we caught the breakpoint.
/// We use lock-free, volatile writes to ensure this works even in edge cases.
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // We ignore the stack frame (_) because we're just displaying a message
    // and not doing complex exception recovery.
    
    unsafe {
        // VGA text mode memory starts at physical address 0xb8000
        let base = 0xb8000 as *mut u8;
        
        // In VGA text mode, each character takes 2 bytes:
        //   Byte 0: ASCII character code
        //   Byte 1: Color (high 4 bits = background, low 4 bits = foreground)
        // Format: [background << 4 | foreground]
        // Black (0) background, LightRed (12) foreground = 0x0C
        let color: u8 = (0 << 4) | (crate::vga_buffer::Color::LightRed as u8);
        
        // The message we'll write
        let text = b"BRKP";
        
        // Write each character to VGA memory
        for (i, &ch) in text.iter().enumerate() {
            // Each character occupies 2 bytes (char + color), so offset = i * 2
            let offset = i * 2;
            
            // write_volatile ensures the CPU actually writes to the address
            // (prevents compiler optimizations from removing the write)
            core::ptr::write_volatile(base.add(offset), ch);
            core::ptr::write_volatile(base.add(offset + 1), color);
        }
    }
}

/// Handler for double fault exceptions (exception #8).
///
/// A double fault occurs when the CPU tries to handle an exception,
/// but that handler itself causes another exception. For example:
/// - CPU tries to handle a page fault
/// - The page fault handler has a bug and causes another exception
/// - Result: double fault (CPU's error recovery mechanism)
///
/// This is serious—triple faults cause the system to reboot.
/// We use a dedicated IST stack (configured in gdt.rs) to ensure we have
/// a valid stack even if the kernel stack overflowed and caused the original fault.
///
/// This function never returns (marked with `-> !`), because after a double fault,
/// the kernel is in an unstable state.
///
/// # Parameters
/// - `stack_frame`: The CPU state when the double fault occurred
/// - `_error_code`: Error information (ignored here)
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    // Display "DOUBLE FAULT" in red on the screen
    unsafe {
        let base = 0xb8000 as *mut u8;
        // Red foreground (12 << 4 is not used here, we use 12 as the color value directly)
        // Format: [background << 4 | foreground] = [0 << 4 | 12] = 0x0C
        let color: u8 = (0 << 4) | (crate::vga_buffer::Color::Red as u8);
        let text = b"DOUBLE FAULT!";
        
        for (i, &ch) in text.iter().enumerate() {
            let offset = i * 2; // 2 bytes per character (char + color)
            core::ptr::write_volatile(base.add(offset), ch);
            core::ptr::write_volatile(base.add(offset + 1), color);
        }
    }
    
    // Panic with the CPU state information for debugging
    // {:#?} uses the pretty-print debug format to show all registers clearly
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}