
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use ferret_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_should_fail();
    serial_println!(" -FAILED-");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn test_should_fail() {
    serial_print!("test_should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!(" -PASSED-");
    exit_qemu(QemuExitCode::Success);
    loop {}
}