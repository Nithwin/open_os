//! VGA text buffer module for bare-metal OS output.
//!
//! This module provides a writer interface to directly manipulate VGA text-mode memory
//! at 80×25 characters (physical address 0xb8000 on x86). Each cell on screen stores
//! one ASCII byte plus one color byte.

use core::fmt;
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// VGA text mode supports 16 colors, each stored as a 4-bit value.
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
/// Packs foreground and background color into a single byte.
///
/// VGA stores color as: [background (4 bits) | foreground (4 bits)]
/// This struct wraps that 8-bit layout for type safety.
pub struct ColorCode(u8);

impl ColorCode {
    /// Combines foreground and background colors into one packed byte.
    ///
    /// # Arguments
    /// * `foreground` - the text color (0-15)
    /// * `background` - the screen color behind text (0-15)
    ///
    /// # How it works
    /// - Background is shifted left 4 bits to occupy bits [7:4]
    /// - Foreground occupies bits [3:0]
    /// - Example: background=Blue(1), foreground=White(15)
    ///   - Blue << 4 = 0001_0000 (16)
    ///   - 16 | 15 = 0001_1111 (31)
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
/// One cell on the VGA text screen.
///
/// Layout matches VGA hardware: ASCII byte first, color byte second.
/// #[repr(C)] ensures predictable field order in memory.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
/// The full 80×25 VGA text buffer.
///
/// Maps directly to physical memory at 0xb8000 on x86.
/// Each cell is 2 bytes (ASCII + color), so total = 80 * 25 * 2 = 4000 bytes.
/// #[repr(transparent)] ensures this struct has the same layout as the inner array.
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Stateful writer for text output to VGA buffer.
///
/// Tracks cursor position, current color, and the underlying buffer reference.
/// Handles wrapping, newlines, and scrolling automatically.
pub struct Writer {
    pub column_position: usize,
    pub row_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    /// Write a single byte to the buffer at the current cursor position.
    ///
    /// If byte is `\n`, moves to the next line.
    /// If cursor reaches end of row, wraps to next row.
    /// If at bottom row and newline is triggered, scrolls screen up.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Write a string to the buffer.
    ///
    /// Only printable ASCII (0x20–0x7e) and newline are written directly.
    /// Non-printable bytes are replaced with 0xfe (a visible glyph).
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Move cursor to the next line.
    ///
    /// If not at bottom row, increment row and reset column to 0.
    /// If at bottom row, scroll all lines up by one and clear the last row.
    fn new_line(&mut self) {
        self.column_position = 0;

        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            // Scroll: copy each row upward by one position.
            for row in 1..BUFFER_HEIGHT {
                let character = self.buffer.chars[row];
                self.buffer.chars[row - 1] = character;
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
    }

    /// Clear a row by filling it with spaces in the current color.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

/// Implement Rust's `fmt::Write` trait for formatted printing.
///
/// This allows the Writer to work with Rust formatting macros like `write!` and `writeln!`.
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
