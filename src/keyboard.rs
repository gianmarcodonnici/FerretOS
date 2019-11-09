use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts, KeyEvent, KeyState};
use spin::Mutex;
use lazy_static::lazy_static;

const ERROR_CHAR: char = '!';

//PS/2 Keyboard object
lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, pc_keyboard::HandleControl::Ignore));
}

//PS/2 Keyboard scancode translator
pub fn generate_key_event(byte_read: u8) -> KeyEvent {
    let mut keyboard = KEYBOARD.lock();
    match keyboard.add_byte(byte_read).unwrap() {
        Some(key_event) => return key_event,
        None => panic!("key event generation failed"),
    }
}

pub fn key_event_to_char(key_event: KeyEvent) -> char {
    if key_event.state == KeyState::Up {
        return ' ';
    }

    let mut keyboard = KEYBOARD.lock();
    match keyboard.process_keyevent(key_event).unwrap() {
        DecodedKey::Unicode(key) => return key,
        DecodedKey::RawKey(_key) => return ERROR_CHAR,
    }
}

pub fn get_char_from_bytes(bytes: u8) -> char {
    key_event_to_char(generate_key_event(bytes))
}