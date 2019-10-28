#![no_std]      // no std where we're going kid!
#![no_main]     // main is for suckers, disabled for a custom entry point

/// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // don't mangle, the linker will look for a function named _start
pub extern "C" fn _start() -> ! {   // use the C calling convention
    loop {}
}
