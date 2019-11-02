
// CPU Exception handling
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}


pub fn init_idt() {    //Create idt and load it
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    // extern because interrupt handlers have a special calling convention
    println!("EXCEPTION: BREAKPOINT\n {:#?}", stack_frame);
}

