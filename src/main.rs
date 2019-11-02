#![no_std]      //no std where we're going kid!
#![no_main]     //main is for suckers, disabled for a custom entry point
#![feature(custom_test_frameworks)] //use a custom test custom_test_framework
#![test_runner(ferret_os::test_runner)] //enable test runner
#![reexport_test_harness_main = "test_main"] //test usually run in main, but we dont use main

use ferret_os::println;

//This function is called on panic, while not in test mode
use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC {}", _info);
    loop {}
}

//Panic function, in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ferret_os::test_panic_handler(info);
}

#[no_mangle]    //don't mangle, the linker will look for a function named _start
pub extern "C" fn _start() -> ! {   //use the C calling convention
    println!("Hello World");

    ferret_os::init();

    #[cfg(test)] // Run test if in test mode
    test_main();

    loop {}
}
