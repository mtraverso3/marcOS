// Implement the core::fmt::Write trait for Writer
// This allows us to use the write! macro
use core::fmt;

use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

// Enum to represent the VGA colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

// Color code struct, includes foreground and background colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Combine the character and color code into a struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] //ensure ascii_character is first, then color_code (C-like)
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// The VGA buffer struct
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


// We have a Writer struct that will write to the VGA buffer

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Implement the Writer struct

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        // Handle newlines
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // If we're at the end of the line, go to the next line
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // Get the row and column position
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // Get the color code
                let color_code = self.color_code;

                // Write the byte to the VGA buffer
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // Increment the column position
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        // Loop through each byte in the string, and write it to the VGA buffer
        for byte in s.bytes() {
            match byte {
                // ASCII printable bytes or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Anything else is a non-printable character
                _ => self.write_byte(0xfe), // 0xfe is the ■ character
            }
        }
    }


    fn new_line(&mut self) {
        // Loop through all the rows, and move each row up one
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();

                // Move the character up one row
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        // Clear the last row
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row_num: usize) {
        // Get the blank character
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        // Loop through all the columns, and set the character to blank
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row_num][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// create a public global writer instance
// This is a lazy_static, which means it will be initialized the first time it's used
// We use a Mutex to ensure that only one thread can access the writer at a time (since VGA is a shared resource)
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Macro to print to the VGA buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_screen::_print(format_args!($($arg)*)));
}


// Println macro, calls print! macro and then prints a newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Directly use the Writer struct to print to the VGA buffer
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}