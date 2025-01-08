#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use core::slice;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    vga_buffer::print_something();
    loop {}
}
