// Rust 2024 edition requires import these derive traits
use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::prelude::rust_2024::derive;

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
//
// Here we have 2 non-zero fields. repr(C) is the better choice.
// TODO Why should we not use repr(C) everywhere?
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
    chars: [[SreeenChar; BUFFER_WITDH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            _ => {
                if self.column_position >= BUFFER_WITDH {
                    self.newline()
                }

                self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position] = SreeenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };

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
                self.buffer.chars[row - 1][column] = self.buffer.chars[row][column]
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0
    }

    fn clear_row(&mut self, row: usize) {
        for column in 0..BUFFER_WITDH {
            self.buffer.chars[row][column].ascii_char = b' ';
        }
    }
}
