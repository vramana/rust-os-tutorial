// Rust 2024 edition requires import these derive traits
use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt;
use core::fmt::Debug;
use core::marker::Copy;
use core::prelude::rust_2024::derive;

use core::ptr::NonNull;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::access::ReadWrite;
use volatile::VolatileRef;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// repr(transparent) doesn't work here. According to documentation, it can only
// used on struct or single variant enum with a single non zero sized field
// https://doc.rust-lang.org/nomicon/other-reprs.html?highlight=align#reprtransparent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct SreeenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WITDH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    // TODO Modify this to [[ScreenChar]] instead of [ScreenChar] since it simplifies the writer code.
    chars: VolatileRef<'static, [SreeenChar], ReadWrite>,
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {
            Buffer {
                chars: VolatileRef::new(NonNull::from(
                    // What' more appropriate *mut ScreenChar or *const ScreenChar? Both works here
                    // Is the check suppressed due to unsafe block?
                    //
                    // TODO Is there no way to extract this address into static variable?
                    core::slice::from_raw_parts(0xb8000 as *mut SreeenChar, 80 * 25),
                )),
            }
        },
    });
}

fn get_buffer_index(row: usize, column: usize) -> usize {
    row * BUFFER_WITDH + column
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            _ => {
                if self.column_position >= BUFFER_WITDH {
                    self.newline()
                }

                let last_row = BUFFER_HEIGHT - 1;
                let position = get_buffer_index(last_row, self.column_position);

                let row = self.buffer.chars.as_mut_ptr().index(position);

                row.write(SreeenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for b in string.bytes() {
            match b {
                0x20..0x7e | b'\n' => self.write_byte(b),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn newline(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WITDH {
                let position = get_buffer_index(row, column);
                let ch = self.buffer.chars.as_ptr().index(position).read();
                self.buffer
                    .chars
                    .as_mut_ptr()
                    .index(get_buffer_index(row - 1, column))
                    .write(ch);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0
    }

    fn clear_row(&mut self, row: usize) {
        for column in 0..BUFFER_WITDH {
            let position = get_buffer_index(row, column);
            self.buffer
                .chars
                .as_mut_ptr()
                .index(position)
                .write(SreeenChar {
                    ascii_char: b' ',
                    color_code: self.color_code,
                });
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

static HELLO_WORLD: &str = "Hello, world!\n\nHello Sailor!";

pub fn print_something() {
    WRITER.lock().write_string(HELLO_WORLD);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
