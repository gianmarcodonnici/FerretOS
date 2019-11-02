
// CPU Exception handling
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

pub fn init_idt() {    //Create idt
    let mut idt = InterruptDescriptorTable::new();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    // extern because interrupt handlers have a special calling convention
    println!("EXCEPTION: BREAKPOINT\n {:#?}", stack_frame);
}

