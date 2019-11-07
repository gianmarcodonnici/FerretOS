use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
use spin::Mutex;
use lazy_static::lazy_static;


//PS/2 Keyboard scancode translator
lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, pc_keyboard::HandleControl::Ignore));
}

pub fn get_char(scancode: u8) -> char {
    let mut keyboard = KEYBOARD.lock();
    /*let key = kb.process_keyevent(kb.add_byte(scancode));
    let dkey = DecodedKey::Unicode(Some(key));
    match dkey {
        DecodedKey::Unicode(character) => character,
        _ => 'a',
    }*/
    let mut ret = '!';
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => ret = character,
                DecodedKey::RawKey(_key) => ret = '!',
            }
        }
    }
    return ret;
}