//! Global Descriptor Table (GDT) and Task State Segment (TSS) initialization.
//!
//! The GDT is a data structure used in x86/x86_64 to describe memory segments.
//! The TSS (Task State Segment) is used to store CPU state and provide dedicated stacks
//! for interrupt handlers. On x86-64, we mainly use the TSS for its IST (Interrupt Stack Table),
//! which allows critical handlers (like double faults) to run on dedicated stacks.

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

/// Stack index for the double fault handler's IST entry.
///
/// IST (Interrupt Stack Table) in the TSS provides separate stacks for critical interrupt handlers.
/// Index 0 is the first available slot. When the CPU raises a double fault, it will use
/// the stack pointer at this IST index instead of the current (potentially corrupted) stack.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    /// The Task State Segment (TSS) for the kernel.
    ///
    /// The TSS stores CPU state information and provides the IST (Interrupt Stack Table).
    /// We create a dedicated 20 KiB stack for the double fault handler so that even if
    /// the kernel stack overflows/corrupts, we still have a valid stack to handle the error.
    ///
    /// lazy_static ensures the TSS is created exactly once when first accessed.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        
        // Configure the stack for the double fault handler
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // 20 KiB of memory for the double fault handler's stack
            const STACK_SIZE: usize = 4096 * 5;
            
            // Static mutable array to hold the actual stack memory.
            // Safe here: single-threaded kernel with no concurrency.
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            
            #[allow(static_mut_refs)]
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            
            // On x86-64, stacks grow downward (toward lower memory addresses).
            // The stack "top" (where we start pushing) is highest address: start + size.
            stack_start + STACK_SIZE as u64
        };
        tss
    };
}

lazy_static! {
    /// The Global Descriptor Table (GDT) and its associated selectors.
    ///
    /// The GDT is a table that describes memory segments for the CPU.
    /// It contains:
    ///   1. A kernel code segment (defines where kernel code lives)
    ///   2. A TSS segment (points to the Task State Segment we created above)
    ///
    /// We bundle the GDT with the Selectors struct so we have both the table
    /// and the selector values needed to reload the CPU's segment registers.
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        
        // Add kernel code segment descriptor to the GDT
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        
        // Add TSS descriptor to the GDT (references the TSS we created above)
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        
        // Return both the GDT and the selectors we'll need to use it
        (gdt, Selectors { code_selector, tss_selector })
    };
}

/// Holds the CPU segment selectors needed after loading the GDT.
///
/// After loading a new GDT, the CPU still uses the old segment registers.
/// We need to reload them to point to the new GDT entries:
///   - cs (code segment register) → must point to our kernel code segment
///   - tr (task register) → must point to our TSS segment
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/// Load the GDT and reload the CPU segment registers.
///
/// This function performs the critical step of telling the CPU about our new GDT and TSS.
/// After loading a new GDT:
///   1. We load the GDT into the CPU's GDTR register
///   2. We reload cs (code segment) to point to OUR code segment descriptor
///   3. We load tr (task register) to point to OUR TSS descriptor
///
/// Without reloading these registers, the CPU still uses the old bootloader GDT entries,
/// and our new TSS (with its dedicated double fault stack) would never be used.
///
/// # Safety
/// This is unsafe because we're telling the CPU to use new memory locations for critical
/// structures. Must only be called during kernel initialization when no interrupts occur.
pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    // Step 1: Load our GDT into the CPU's GDTR register
    GDT.0.load();
    
    // Step 2 & 3: Update CPU segment registers to use our new GDT entries
    unsafe {
        // Reload the code segment register to point to our kernel code segment
        CS::set_reg(GDT.1.code_selector);
        
        // Load the task register to point to our TSS
        // This tells the CPU where to find stack pointers for exceptions
        load_tss(GDT.1.tss_selector);
    }
}
