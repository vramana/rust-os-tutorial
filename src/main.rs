#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO_WORLD: &str = "Hello, world!\n\nHello Sailor";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string(HELLO_WORLD);

    loop {}
}
