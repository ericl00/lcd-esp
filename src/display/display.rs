use heapless::String;
use crate::display::display;
use crate::display::drivers::Drivers;
use crate::display::drivers::Pins;
use crate::display::drivers::DataLength;
use crate::display::instructions::Instructions;

pub struct Display<'a> {
    text: String<64>,
    next_instruction: Option<u8>,
    drivers: Drivers<'a>,
    initialized: bool,
    cursor_ptr: u8,
}

impl<'a> Display<'a> {
    pub fn new(pins: Pins) -> Self {
        let text = String::new();
        let drivers = Drivers::new(pins);

        let mut display = Display {
            text,
            next_instruction: None,
            drivers,
            initialized: false,
            cursor_ptr: 0,
        };
        display.init();
        display
    }

    fn init(&mut self) {
        match self.drivers.mode {
            DataLength::Bits8 => {
                self.instruction(Instructions::Initialize8BitMode2Line5x10);
            },
            DataLength::Bits4 => {
                self.instruction(Instructions::Initialize4BitMode2Line5x10);
            },
        }
        self.initialized = true;
    }

    fn instruction(&mut self, instruction: Instructions) {
        self.drivers.write(instruction.value());
    }

    fn push_str(&mut self, text: &str) {
        let chars = text.chars();
        self.cursor_ptr += text.len() as u8;
        self.drivers.write_text(chars);
        self.text.push_str(text).unwrap();
    }
}
