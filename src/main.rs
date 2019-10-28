#![no_std]      // no std where we're going kid!
#![no_main]     // main is for suckers

/// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]    // keep the name, otherwise rust renames this function to a unique name
pub extern "C" fn _start() -> ! {   // use the C calling convention, to have a valid entry point
    loop {}
}
