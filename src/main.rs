#![no_std]      // no std where we're going kid!
#![no_main]     // main is for suckers, disabled for a custom entry point

/// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // don't mangle, the linker will look for a function named _start
pub extern "C" fn _start() -> ! {   // use the C calling convention
    let vga_buffer = 0xb8000 as *mut u8;    // vga buffer is at this address
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
