use core::str::Chars;
use esp_hal::delay::Delay;
use esp_hal::gpio::Level::Low;
use esp_hal::gpio::{Flex, Output, OutputConfig};
use esp_hal::gpio::AnyPin;
use crate::display::instructions::Instructions;

pub(crate) enum DataLength {
    Bits4,
    Bits8,
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
    pub(crate) mode: DataLength,
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

        if d0.is_none() && d1.is_none() && d2.is_none() && d3.is_none() {
            mode = DataLength::Bits4;
        } else if d0.is_none() || d1.is_none() || d2.is_none() || d3.is_none() {
            panic!()
        }
        Pins {
            rs, rw, e, d7, d6, d5, d4, d3, d2, d1, d0, mode
        }
    }
}

pub(crate) struct Drivers<'a> {
    rsoutput: Output<'a>,
    rwoutput: Output<'a>,
    eoutput: Output<'a>,
    eight_bits: Option<[Flex<'a>; 8]>,
    four_bits: Option<[Flex<'a>; 4]>,
    delay: Delay,
    pub mode: DataLength,
}

impl<'a> Drivers<'a> {
    pub(crate) fn new(mut pins: Pins) -> Self {
        let rsoutput = Output::new(pins.rs, Low, OutputConfig::default());
        let rwoutput = Output::new(pins.rw, Low, OutputConfig::default());
        let eoutput = Output::new(pins.e, Low, OutputConfig::default());
        let d4io = Flex::new(pins.d4);
        let d5io = Flex::new(pins.d5);
        let d6io = Flex::new(pins.d6);
        let d7io = Flex::new(pins.d7);
        match pins.mode {
            DataLength::Bits8 => {
                let d3io = Flex::new(pins.d3.take().unwrap());
                let d2io = Flex::new(pins.d2.take().unwrap());
                let d1io = Flex::new(pins.d1.take().unwrap());
                let d0io = Flex::new(pins.d0.take().unwrap());
                Drivers {
                    rsoutput, rwoutput, eoutput, eight_bits: Some([d7io, d6io, d5io, d4io, d3io, d2io, d1io, d0io]), four_bits: None, delay: Delay::new(), mode: DataLength::Bits8,
                }
            }
            DataLength::Bits4 => {
                Drivers {
                    rsoutput, rwoutput, eoutput, eight_bits: None, four_bits: Some([d7io, d6io, d5io, d4io]), delay: Delay::new(), mode: DataLength::Bits4,
                }
            }
        }
    }

    fn data_pins_set<I>(&mut self, data: &u8, index: u8, pins: I)
    where
        I: Iterator<Item = &'a mut Flex<'a>>
    {
        self.eoutput.set_high();
        for (n, pin) in pins.enumerate() {
            if (data >> (index - n as u8)) & 1 == 1 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
        self.delay.delay_micros(20);
        self.eoutput.set_low();
    }

    pub(crate) fn write(&mut self, data: u8) {
        match self.mode {
            DataLength::Bits8 => {
                let drv_ptr = self.eight_bits.as_mut().map(|drv| drv as *mut [Flex<'a>; 8]);

                if let Some(drv_ptr) = drv_ptr {
                    let drv = unsafe { &mut *drv_ptr };
                    self.data_pins_set(&data, 7, drv.iter_mut());
                }
            },
            DataLength::Bits4 => {
                let drv_ptr = self.four_bits.as_mut().map(|drv| drv as *mut [Flex<'a>; 4]);

                if let Some(drv_ptr) = drv_ptr {
                    unsafe {
                        self.data_pins_set(&data, 7, (&mut *drv_ptr).iter_mut());
                    }
                    self.delay.delay_micros(50);
                    unsafe {
                        self.data_pins_set(&data, 3, (&mut *drv_ptr).iter_mut());
                    }
                }
            }
        }
    }
    pub(crate) fn write_text(&mut self, data: Chars) {
        for ch in data {
            self.rsoutput.set_high();
            self.delay.delay_micros(20);
            self.write(ch as u8);
            self.delay.delay_micros(20);
            self.rsoutput.set_low();
            self.delay.delay_micros(5);
            self.write(Instructions::ShiftCursorRight.value());
        }
    }
}