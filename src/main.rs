#![no_std]      //no std where we're going kid!
#![no_main]     //main is for suckers, disabled for a custom entry point
#![feature(custom_test_frameworks)] //use a custom test custom_test_framework
#![test_runner(crate::test_runner)] //enable test runner
#![reexport_test_harness_main = "test_main"] //test usually run in main, but we dont use main

mod vga_buffer;

//This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC {}", _info);
    loop {}
}

#[no_mangle]    //don't mangle, the linker will look for a function named _start
pub extern "C" fn _start() -> ! {   //use the C calling convention
    println!("Hello World");

    #[cfg(test)] // Run test if in test mode
    test_main();

    loop {}
}

//Test framework
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {   //Rust collects all tests and passes them to this function
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// Test testing (heh)
#[test_case]
fn test_test() {
    print!("Testing tests... ");
    assert_eq!(1,1);
    println!("-PASSED-")
}
