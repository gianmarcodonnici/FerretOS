//Testing the stack overflow
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use ferret_os::serial_print;
use core::panic::PanicInfo;

//Create a custom idt for this test

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;


lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(ferret_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

//Custom double fault handler
use ferret_os::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    serial_println!(" -PASSED-");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("test_stack_overflow...");
    ferret_os::gdt::init();
    init_test_idt();

    //Stack overflow is a go!
    stack_overflow();

    panic!("Execution continued after a stack overflow. This isn't right.")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ferret_os::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}