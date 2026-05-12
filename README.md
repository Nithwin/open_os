# OpenOS - The Open-Source OS Learning Repository

A minimal, educational bare-metal operating system kernel written in Rust for x86 architecture. OpenOS is designed to be the **best learning resource on GitHub** for understanding how operating systems work from the ground up. This project demonstrates fundamental OS concepts like direct hardware manipulation, VGA text-mode I/O, and bootloading with a focus on clarity, documentation, and accessibility for learners at all levels.

## 🎯 Project Overview

OpenOS is designed from the ground up to be:
- **Accessible**: Well-commented code explaining every decision and concept
- **Educational**: Minimal implementation showing core OS principles without unnecessary complexity
- **Hands-on**: Encourages learning by building, modifying, and experimenting
- **Community-driven**: Open source for contributions, suggestions, and improvements
- **Bare metal**: Runs without a standard library (`#![no_std]`) directly on x86 CPU
- **Transparent**: Output via VGA text-mode memory (0xb8000) with clear explanations

**Not a production OS**—this is a learning project designed to demystify operating systems and inspire the next generation of OS developers.

## 📁 Project Structure

```
open_os/
├── src/
│   ├── main.rs              # Kernel entry point (_start function)
│   └── vga_buffer.rs        # VGA text buffer driver and writer
├── Cargo.toml               # Rust project manifest
├── Cargo.lock               # Dependency lock file
├── x86_64-nyx.json          # Custom x86_64 target specification
├── .cargo/
│   └── config.toml          # Cargo configuration (uses bootimage runner)
└── target/                  # Compiled artifacts (build output)
```

## 🔑 Key Components

### `main.rs` - Kernel Entry Point
- **Purpose**: The first code executed after bootloader hands control to the kernel
- **What it does**:
  - Initializes the VGA writer with yellow text on black background
  - Maps physical memory address 0xb8000 (VGA text buffer) into Rust memory
- Prints boot messages demonstrating both direct string output and Rust's formatted `write!` macro
- Spins in an infinite loop (placeholder for interrupt handling and real OS scheduler)
  - `#[no_std]` - no standard library (need raw hardware access)
  - `#[no_main]` - define custom entry point, not the normal Rust `main()`
  - `#[unsafe(no_mangle)]` + `extern "C"` - expose symbol for bootloader to call
  - Unsafe pointer cast to VGA memory: `0xb8000 as *mut vga_buffer::Buffer`

### `vga_buffer.rs` - VGA Text-Mode Driver
The core of text output. Direct hardware manipulation for screen printing.

#### Key Types:

**`Color` enum** (lines 12-28)
- All 16 VGA colors (Black, Blue, Green, Cyan, Red, Magenta, Brown, LightGray, DarkGray, LightBlue, LightGreen, LightCyan, LightRed, Pink, Yellow, White)
- Each color is a 4-bit value (0-15)
- `#[repr(u8)]` ensures it maps directly to hardware color codes

**`ColorCode` struct** (lines 30-57)
- Packs two colors (foreground + background) into one byte
- Layout: `[background (4 bits) | foreground (4 bits)]`
- Example: Yellow text (15) on Black background (0) = 0x0F ( binary: 0000_1111)
- `#[repr(transparent)]` means it has the same memory layout as u8

**`ScreenChar` struct** (lines 59-66)
- One cell on the VGA text screen
- Contains: ASCII byte + color byte
- `#[repr(C)]` ensures predictable memory layout matching hardware expectations

**`Buffer` struct** (lines 75-81)
- The full 80×25 grid of screen characters
- 80 columns × 25 rows = 2000 cells
- Total size: 80 × 25 × 2 bytes = 4000 bytes
- Maps directly to physical memory at 0xb8000
- `#[repr(transparent)]` means the struct has identical memory layout to its inner array

**`Writer` struct** (lines 83-89)
- Stateful text writer that tracks cursor position and color
- Fields:
  - `column_position` - current x coordinate (0-79)
  - `row_position` - current y coordinate (0-24)
  - `color_code` - text color to use for new characters
  - `buffer` - mutable reference to the VGA buffer (lifetime `'static`)

#### Key Methods:

**`write_byte()` (lines 93-118)**
- Writes a single character at current cursor position
- Handles special case: newline (`\n`) triggers line break
- Auto-wraps to next row if you reach column 80
- Increments column position after writing

**`write_string()` (lines 120-128)**
- Writes an entire string byte-by-byte
- Filters: only prints printable ASCII (0x20–0x7e) and newline
- Replaces non-printable bytes with 0xfe (visible glyph)

**`new_line()` (lines 130-150)**
- Moves cursor to next line
- **Normal case** (not at bottom): just increment row + reset column
- **Bottom case** (at row 24): **scroll screen up**
  - Copy every row upward by one position: row[n-1] = row[n]
  - Clear the last row (now empty)
  - This creates the illusion of scrolling

**`clear_row()` (lines 152-160)**
- Fills a row with space characters
- Used to clear the last row after scrolling

**`fmt::Write` trait implementation (lines 162-168)**
- Enables Rust's formatted printing macros: `write!()`, `writeln!()`
- Bridges Rust's formatting system with hardware text output

## 🚀 Building & Running

### Prerequisites
- **Rust** (stable or nightly, recommend nightly for better bare-metal support)
- **Cargo** (comes with Rust)
- **bootimage** (bootloader runner): `cargo install bootimage`
- **QEMU** (x86_64 emulator): `sudo apt install qemu-system-x86` (Ubuntu/Debian)

### Build
```bash
cd /home/shadow/Desktop/open_os
cargo build
```

### Run in QEMU
```bash
cargo run
```

This compiles the kernel and runs it in QEMU emulator. You should see:
```
Hello this is Nithwin
Welcome to NyxOS Version 1!
Booting from Bare Metal... Hex test: 0xDEADBEEF
```

in yellow text on a black background.

### Build Only (without running)
```bash
cargo build --release
```

## 💡 Learning Concepts

### Bare Metal
- **No OS underneath**: The kernel runs directly on CPU with no abstraction layer
- **Direct hardware access**: Map physical memory, read/write I/O ports, configure CPU
- **Bootloader responsibility**: Sets up minimal CPU state (protected mode, stack, memory)

### VGA Text Mode
- **Historical legacy**: IBM PC standard from the 1980s
- **Memory-mapped I/O**: Screen controlled by writing to memory address 0xb8000
- **Simple cell format**: Each cell = 1 byte (ASCII) + 1 byte (color)
- **No abstraction**: Changes to 0xb8000 appear immediately on screen

### Memory Representation
VGA color byte layout:
```
Bit Layout: [7:4 background color] [3:0 foreground color]
Example:    0000_1111 = Black bg, White fg
            1111_0000 = White bg, Black fg
```

ScreenChar byte layout (two consecutive bytes in memory):
```
Byte 1: ASCII character (0-255)
Byte 2: Color code (bit pattern above)
```

### Rust `no_std` Constraints
- **No heap allocations** by default (no `Box`, `Vec`, `String`)
- **No panic unwinding** (need custom panic handler)
- **Raw pointers required** to access hardware memory
- **Unsafe blocks necessary** for memory-mapped I/O
- **Manual memory management** for static arrays

### Why the Complexity?
- `#[repr(u8)]` / `#[repr(C)]` / `#[repr(transparent)]` - ensure Rust structs match hardware byte layouts
- Strict field order - hardware expects specific binary layout, no flexibility
- `'static` lifetime - VGA buffer lives for entire program lifetime
- `unsafe` blocks - Rust can't verify hardware access is safe, programmer must ensure it

## 📚 Documentation

Every function, struct, and module in the source code includes detailed doc comments explaining:
- **What** it does
- **Why** it exists
- **How** it works internally
- **Examples** where relevant

Read the inline comments in [src/vga_buffer.rs](src/vga_buffer.rs) and [src/main.rs](src/main.rs) for step-by-step explanations.

## 🔄 Scrolling Example

When text reaches the bottom of the screen, OpenOS handles scrolling by shifting all content upward:

**Before scroll:**
```
Row 0:  "AAA..."
Row 1:  "BBB..."
...
Row 23: "YYY..."
Row 24: "ZZZ..."  <- Last row, no room for new text
```

**After `new_line()` at bottom:**
```
Row 0:  "BBB..."  (was row 1)
Row 1:  "CCC..."  (was row 2)
...
Row 23: "ZZZ..."  (was row 24)
Row 24: "[blank]" (cleared for new text)
```

The `write_byte()` function then writes the next character at row 24, column 0. This mimics terminal behavior you'd see in any OS!

## 🎓 Next Steps for Learning

1. **Experiment with colors**: Modify `ColorCode::new()` arguments in `main.rs` to try different text/background combinations
2. **Add new print functions**: Write helpers like `print_centered()` or `print_box()`
3. **Track what gets printed**: Add a counter that increments each time a character is written
4. **Handle more escape codes**: Extend `write_byte()` to support `\r` (carriage return), `\t` (tab), etc.
5. **Interrupt handling**: Add keyboard input or timer interrupts to respond to hardware events
6. **Allocator**: Implement a memory allocator to enable heap allocations in no_std contexts

## 📖 File Details

| File | Lines | Purpose |
|------|-------|---------|
| `src/main.rs` | ~60 | Kernel entry point, boot messages |
| `src/vga_buffer.rs` | ~170 | VGA driver, text output logic |
| `Cargo.toml` | ~20 | Project metadata, dependencies |
| `x86_64-nyx.json` | ~20 | Custom x86_64 target config |

## ⚙️ Technical Notes

- **Target**: x86_64 custom target (defined in `x86_64-nyx.json`)
- **Boot method**: BIOS → bootloader (via bootimage crate) → kernel
- **Default output**: VGA text mode (80×25 characters, cyan text on black)
- **Runtime environment**: QEMU x86_64 CPU emulator
- **Memory model**: Real mode → protected mode (handled by bootloader)

## 🛠️ Customization

### Change Text Color
In `src/main.rs`, line 41:
```rust
// Change Yellow to any Color enum variant
color_code: vga_buffer::ColorCode::new(vga_buffer::Color::Green, vga_buffer::Color::Black),
```

### Change Boot Message
In `src/main.rs`, lines 49-55:
```rust
writer.write_string("Your custom text here\n");
write!(writer, "Add dynamic content: {}", some_variable).unwrap();
```

### Modify Screen Size
In `src/vga_buffer.rs`, lines 71-72 (requires bootloader reconfiguration):
```rust
const BUFFER_HEIGHT: usize = 50;  // Requires VGA mode change
const BUFFER_WIDTH: usize = 80;
```

## 📝 License

OpenOS is released under the **MIT License**. See [LICENSE](LICENSE) for details.

You're free to:
- ✅ Use this for learning
- ✅ Fork and modify
- ✅ Contribute improvements back
- ✅ Use in your own projects
- ✅ Share with others

Just include the license notice. This is truly open source!

---

## 🤝 Contributing

OpenOS thrives on community contributions! If you:
- Find a confusing explanation → Clarify it
- Spot a bug → Fix it
- Have a great learning feature → Add it
- Want to improve documentation → We'd love it!

See the issues and feel free to open pull requests. Let's build the best OS learning resource together!

## 📞 Questions & Discussions

Confused about a concept? Check:
1. **Inline code comments** - Every function has detailed explanations
2. **README sections** - Specific learning concepts explained here
3. **GitHub Issues** - Ask questions, we'll help!

---

**Happy OS hacking! 🚀**

Made with ❤️ for OS learners everywhere.
