/// Instructions
const CLEAR: u8 = 0x01;
const RETURN_HOME: u8 = 0x02;
const ENTRY_MODE_SET: u8 = 0x04;
const DISPLAY_ON_OFF: u8 = 0x08;
const CURSOR_OR_DISPLAY_SHIFT: u8 = 0x10;
const FUNCTION_SET: u8 = 0x20;
const SET_CGRAM_ADDRESS: u8 = 0x40;
const SET_DDRAM_ADDRESS: u8 = 0x80;


/// Sub Instructions
const ENTRY_MODE_INCREMENT: u8 = 0x02;
const ENTRY_MODE_DECREMENT: u8 = 0x00;
const ENTRY_MODE_SHIFT_RIGHT: u8 = 0x00;
const ENTRY_MODE_SHIFT_LEFT: u8 = 0x01;

const DISPLAY_ON: u8 = 0x04;
const DISPLAY_OFF: u8 = 0x00;
const DISPLAY_CURSOR_ON: u8 = 0x02;
const DISPLAY_CURSOR_OFF: u8 = 0x00;
const DISPLAY_BLINK_ON: u8 = 0x01;
const DISPLAY_BLINK_OFF: u8 = 0x00;

const DISPLAY_SHIFT: u8 = 0x08;
const CURSOR_MOVE: u8 = 0x00;
const RIGHT: u8 = 0x04;
const LEFT: u8 = 0x00;

const FUNCTION_SET_8BIT: u8 = 0x10;
const FUNCTION_SET_4BIT: u8 = 0x00;
const FUNCTION_SET_1LINE: u8 = 0x00;
const FUNCTION_SET_2LINE: u8 = 0x08;
const FUNCTION_SET_FONT5X10: u8 = 0x04;
const FUNCTION_SET_FONT5X8: u8 = 0x00;

#[repr(u8)]
pub enum Instructions {
    Initialize4BitMode2Line5x10 = FUNCTION_SET | FUNCTION_SET_4BIT | FUNCTION_SET_2LINE | FUNCTION_SET_FONT5X10,
    Initialize8BitMode2Line5x10 = FUNCTION_SET | FUNCTION_SET_8BIT | FUNCTION_SET_2LINE | FUNCTION_SET_FONT5X10,
    ShiftCursorRight = CURSOR_OR_DISPLAY_SHIFT | CURSOR_MOVE | RIGHT,
    ShiftCursorLeft = CURSOR_OR_DISPLAY_SHIFT | CURSOR_MOVE | LEFT,
}

impl Instructions {
    pub fn value(self) -> u8 {
        self as u8
    }
}
