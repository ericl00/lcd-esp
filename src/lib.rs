#![no_std]

use core::ops::Index;
use heapless::String;
use esp_hal::gpio::{AnyPin, Output, OutputConfig, Flex};
use esp_hal::gpio::Level::Low;

// Instructions
const CLEAR: u8 = 0x01;
const RETURN_HOME: u8 = 0x02;
const ENTRY_MODE_SET: u8 = 0x04;
const DISPLAY_ON_OFF: u8 = 0x08;
const CURSOR_OR_DISPLAY_SHIFT: u8 = 0x10;
const FUNCTION_SET: u8 = 0x20;
const SET_CGRAM_ADDRESS: u8 = 0x40;
const SET_DDRAM_ADDRESS: u8 = 0x80;


// Sub Instructions
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

enum DataLength {
    Bits4,
    Bits8,
}


// TODO rewrite this
struct Drivers {
    rsoutput: Output<'_>,
    rwoutput: Output<'_>,
    eoutput: Output<'_>,
    d7io: Flex<'_>,
    d6io: Flex<'_>,
    d5io: Flex<'_>,
    d4io: Flex<'_>,
    d3io: Option<Flex<'_>>,
    d2io: Option<Flex<'_>>,
    d1io: Option<Flex<'_>>,
    d0io: Option<Flex<'_>>,
}

impl Drivers {
    fn new(mut pins: Pins) -> Self {
        let rsoutput = Output::new(&pins.rs, Low, OutputConfig::default());
        let rwoutput = Output::new(&pins.rw, Low, OutputConfig::default());
        let eoutput = Output::new(&pins.e, Low, OutputConfig::default());
        let d4io = Flex::new(&pins.d4);
        let d5io = Flex::new(&pins.d5);
        let d6io = Flex::new(&pins.d6);
        let d7io = Flex::new(&pins.d7);
        let mut d3io = None;
        let mut d2io = None;
        let mut d1io = None;
        let mut d0io = None;
        match pins.mode {
            DataLength::Bits8 => {
                d3io = Some(Flex::new(pins.d3.take().unwrap()));
                d2io = Some(Flex::new(pins.d2.take().unwrap()));
                d1io = Some(Flex::new(pins.d1.take().unwrap()));
                d0io = Some(Flex::new(pins.d0.take().unwrap()));
                Drivers {
                    rsoutput, rwoutput, eoutput, d7io, d6io, d5io, d4io, d3io, d2io, d1io, d0io,
                }
            }
            DataLength::Bits4 => {
                Drivers {
                    rsoutput, rwoutput, eoutput, d7io, d6io, d5io, d4io, d3io, d2io, d1io, d0io,
                }
            }
        }
    }

    fn initialize_4bit_mode(&mut self) {
        self.eoutput.set_high();
        self.rwoutput.set_low();
        let mut pins = [&self.d7io, &self.d6io, &self.d5io, &self.d4io];
        let instruction = FUNCTION_SET | FUNCTION_SET_4BIT;
        for (n, mut pin) in pins.iter().enumerate().map(|(n, pin)| {
            (!(n as u8 | 0b11111000u8), pin)
        }).collect() {
            if (instruction >> n) & 1 == 1 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }
    fn initialize_8bit_mode(&mut self) {

    }
}

pub struct Pins {
    rs: AnyPin,
    rw: AnyPin,
    e: AnyPin,
    d7: AnyPin,
    d6: AnyPin,
    d5: AnyPin,
    d4: AnyPin,
    d3: Option<AnyPin>,
    d2: Option<AnyPin>,
    d1: Option<AnyPin>,
    d0: Option<AnyPin>,
    mode: DataLength,
}

pub struct Display {
    text: String<64>,
    next_instruction: Option<u8>,
    pins: Pins,
    drivers: Drivers
}

impl Pins {
    pub fn new(
        rs: AnyPin,
        rw: AnyPin,
        e: AnyPin,
        d7: AnyPin,
        d6: AnyPin,
        d5: AnyPin,
        d4: AnyPin,
        d3: Option<AnyPin>,
        d2: Option<AnyPin>,
        d1: Option<AnyPin>,
        d0: Option<AnyPin>,
    ) -> Self {
        let mut mode = DataLength::Bits8;
        if &d0 == None && &d1 == None && &d2 == None && &d3 == None {
            mode = DataLength::Bits4;
        } else if &d0 == None || &d1 == None || &d2 == None || &d3 == None {
            panic!()
        }
        Pins {
            rs, rw, e, d7, d6, d5, d4, d3, d2, d1, d0, mode
        }
    }
}

impl Display {
    pub fn new(pins: Pins) -> Self {
        let text = String::new();
        let next_instruction = None;

        Display {
            text,
            next_instruction,
            pins,
        }
    }
}
