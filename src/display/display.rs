use heapless::String;
use crate::display::drivers::Drivers;
use crate::display::drivers::Pins;
use crate::display::instructions::Instructions;

pub struct Display<'a> {
    text: String<64>,
    next_instruction: Option<u8>,
    drivers: Drivers<'a>
}

impl<'a> Display<'a> {
    pub fn new(pins: Pins) -> Self {
        let text = String::new();
        let next_instruction = None;
        let drivers = Drivers::new(pins);

        Display {
            text,
            next_instruction,
            drivers
        }
    }

    pub fn init(&mut self) {}

    fn instruction(&mut self, instruction: Instructions) {
        self.drivers.write(instruction.value());
    }
}
