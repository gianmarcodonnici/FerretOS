#![no_std]      //no std where we're going kid!
#![no_main]     //main is for suckers, disabled for a custom entry point
#![feature(custom_test_frameworks)] //use a custom test custom_test_framework
#![test_runner(crate::test_runner)] //enable test runner
#![reexport_test_harness_main = "test_main"] //test usually run in main, but we dont use main

mod vga_buffer;
mod serial;

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
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    //exit qemu if all tests are a success
    exit_qemu(QemuExitCode::Success);
}

//Quitting QEMU using isa-debug-exit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Test testing (heh)
#[test_case]
fn test_test() {
    serial_print!("Testing tests... ");
    assert_eq!(1,1);
    serial_println!("-PASSED-")
}
