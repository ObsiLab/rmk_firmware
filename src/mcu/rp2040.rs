#[cfg(debug)]
use defmt::println;

use crate::mcu::MCU;

pub struct RP2040 {
    name: &'static str,
    nb_gpios: u8,
}

// implement methods for the RP2040 struct
impl RP2040 {
    pub fn rpi() {
        #[cfg(debug)]
        println!("RPi!")
    }
}

// implement the MCU trait for the RP2040 struct
impl MCU for RP2040 {
    fn new(name: &'static str, nb_gpios: u8) -> Self {
        Self { name , nb_gpios: 8 }
    }

    fn set_gpio_high(&self, pin: u8) {
        //rp2040_hal.setHigh(pin)
        #[cfg(debug)]
        println!("set_gpio_high: {}", pin)
    }
}
