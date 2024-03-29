
// CPU Exception and interrupt handling
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::print;
use crate::gdt;
use lazy_static::lazy_static;

lazy_static! {  //IDT interrupt handlers are set here
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}


pub fn init_idt() {    //Create idt and load it
    IDT.load();
}

//Exception and Interrupt handlers

//Breakpoint exception
extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    // extern because interrupt handlers have a special calling convention
    println!("\nEXCEPTION: BREAKPOINT\n {:#?}", stack_frame);
}

//Double fault exception
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame,
        _error_code: u64) {
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    crate::hlt_loop();
}

//Timer Interrupt
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame)
{
    pic_eoi(InterruptIndex::Timer);
}

//PS/2 Keyboard Interrupt
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame) 
{
    use x86_64::instructions::port::Port;
    use crate::io_ports;

    let mut port = Port::new(io_ports::KEYBOARD);
    let byte: u8 = unsafe {port.read()};
    //let key = keyboard::ScancodeSet2::get_key_press_from_byte(byte).character;
    //print!("{}", key);
    pic_eoi(InterruptIndex::Keyboard);
}


//PIC
use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;    //Start interupts after the cpu exception ones
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex { //List of interrupt indices
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

fn pic_eoi(interrupt_index: InterruptIndex) { //End of interrupt notifier
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_index.as_u8());
    }
}

