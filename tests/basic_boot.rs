#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ferret_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ferret_os::{println, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    ferret_os::init();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ferret_os::test_panic_handler(info)
}

// VGA Tests

#[test_case]
fn test_println_single() {
    serial_print!("test_println_single...");
    println!("test_println test");
    serial_println!(" -PASSED-");
}

#[test_case]
fn test_println_many() {
    serial_print!("test_println_many...");
    for _ in 0..400 {
        println!("test_println_many test");
    }
    serial_println!(" -PASSED-")
}

#[test_case]
fn test_println_check_output() {
    serial_print!("test_println_check_output...");
    let s = "lorem ipsum dolor amet";
    println!("{}",s);
    for (i,c) in s.chars().enumerate() {
         let screen_char = ferret_os::vga_buffer::Writer::read_char(ferret_os::vga_buffer::BUFFER_HEIGHT - 2,i);
         assert_eq!(screen_char, c);
    }
    serial_println!(" -PASSED-")
}

//Interrupt tests

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!(" -PASSED-")
}