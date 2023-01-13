pub mod rp2040;

trait MCU {
    fn new(name: &'static str, nb_gpios: u8) -> Self;

    fn set_gpio_high(&self, pin: u8);
}
