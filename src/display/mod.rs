pub mod display;
pub mod drivers;
pub mod instructions;


#[cfg(test)]
mod test {
    use super::*;
    use esp_hal::gpio::AnyPin;
    use crate::display::drivers::Pins;

    fn mock_pin() -> AnyPin {
        unsafe { core::mem::zeroed() }
    }

    #[test]
    fn test_pins_4bits() {
        let pins = Pins::new(mock_pin(), mock_pin(), mock_pin(), mock_pin(),
                  mock_pin(), mock_pin(), mock_pin(),
                  None, None, None, None);

        matches!(pins.mode, drivers::DataLength::Bits4);
    }
    #[test]
    fn test_pins_8bits() {
        let pins = Pins::new(mock_pin(), mock_pin(), mock_pin(), mock_pin(),
                             mock_pin(), mock_pin(), mock_pin(),
                             Some(mock_pin()), Some(mock_pin()), Some(mock_pin()), Some(mock_pin()));
        matches!(pins.mode, drivers::DataLength::Bits8);
    }

    #[test]
    #[should_panic]
    fn test_pin_panic() {
        let pins = Pins::new(mock_pin(), mock_pin(), mock_pin(), mock_pin(),
                             mock_pin(), mock_pin(), mock_pin(),
                             Some(mock_pin()), Some(mock_pin()), Some(mock_pin()), None);
    }
}