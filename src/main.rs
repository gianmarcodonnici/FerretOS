#![no_std]  //no std where we're going kid!

/// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    // stuff
}
