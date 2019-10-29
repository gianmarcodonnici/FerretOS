//VGA Buffer implementation

//Colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]             //Align on 8 bits
pub enum Color {        //VGA Colour list
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]    //So it as the same data structure as Color
struct ColorCode(u8);   //Type to represent a full color

impl ColorCode {
    //first 4 bits are the foreground, next 3 are the background
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

//Text Buffer

//Single char in the buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  //Use C ordering, rust ordering is undefined
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//VGA buffer always is 80x25
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//The actual text buffer
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//VGA Buffer writer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

     fn new_line(&mut self) { /* TODO */ }

     pub fn write_string(&mut self, s: &str) {
         for byte in s.bytes() {
             match byte {
                 //Printable asci chars have values from 0x20 to 0x7e
                 0x20..=0x7e | b'\n' => self.write_byte(byte),
                 //else print a square
                 _ => self.write_byte(0xfe),
             }
         }
     }
}
