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

/*#[test_case] DISABLED FOR NOW
fn test_println_check_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    use ferret_os::vga_buffer::WRITER;
    use ferret_os::vga_buffer::BUFFER_HEIGHT;
    
    serial_print!("test_println_check_output...");
    let s = "Lorem ipsum dolor sit amet";
    interrupts::without_interrupts(|| { //run without interrupts and lock the writer
        let mut writer = WRITER.lock(); //so nobody else writes to the buffer
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i,c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });

    serial_println!(" -PASSED-")
}*/

//Interrupt tests

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!(" -PASSED-")
}