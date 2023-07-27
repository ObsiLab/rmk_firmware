/*

*        =================== TEST 3 ======================

*/

//?#![no_std] // don't link the Rust standard library
// #![cfg_attr(not(test), no_std)] // only link the Rust standard library when testing
#![no_main] // see #[entry] below, from cortex_m_rt::entry

use core::convert::Infallible;
use cortex_m_rt::entry;
use embedded_hal as hal; // rather than rp2040_hal ?
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use embedded_time::duration::Milliseconds;
use hal::pac;
//use rp2040_hal as hal; // prefer embedded_hal ?
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::device::keyboard::{
    KeyboardLedsReport, NKROBootKeyboardInterface,
};
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::*;

// ?? ----- :
use embedded_time::clock::Error;
use embedded_time::duration::Fraction;
use embedded_time::Instant;
pub const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000u32);
use crate::hal::Timer; // only with rp2040_hal ?
// ? use hal::Timer;

const USBVID: u16 = 0x1209;
const USBPID: u16 = 0x0001;
// ! need to get PID from pid.codes (VID 0x1209)

pub struct TimerClock<'a> {
    timer: &'a Timer,
}

impl<'a> TimerClock<'a> {
    pub fn new(timer: &'a Timer) -> Self {
        Self { timer }
    }
}

impl<'a> embedded_time::clock::Clock for TimerClock<'a> {
    type T = u32;
    const SCALING_FACTOR: Fraction = SCALING_FACTOR;

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(Instant::new(self.timer.get_counter_low()))
    }
}
// ?? -----

const NBKEYS: usize = 3;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let clock = TimerClock::new(&timer);

    let usb_alloc = UsbBusAllocator::new(usb_bus);

    let mut keyboard = UsbHidClassBuilder::new()
        .add_interface(NKROBootKeyboardInterface::default_config(&clock))
        .build(&usb_alloc);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(USBVID, USBPID))
        .manufacturer("usbd-human-interface-device")
        .product("NKRO Keyboard")
        .serial_number("TEST")
        .build();

    let keys: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        // ! check pins, length must be == NBKEYS, maybe autogenerate ?
        &pins.gpio1.into_pull_up_input(),
        &pins.gpio2.into_pull_up_input(),
        &pins.gpio3.into_pull_up_input(), //* etc
    ];

    let mut input_count_down = timer.count_down();
    input_count_down.start(Milliseconds(1)); // ! check and test 10ms ?

    loop {
        if input_count_down.wait().is_ok() {
            let keys = key_press(keys);
        }

        /*
        let keys = if pin.is_high().unwrap() {
                &[Keyboard::A]
            } else {
                &[Keyboard::NoEventIndicated]
        };
        */

        keyboard.interface().write_report(&keys).ok();

        keyboard.interface().tick().unwrap();

        /* // ??
        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.interface().read_report() {

                Ok(l) => {
                    update_leds(l);
                }
                _ => {}
            }
        }
        */
        keyboard.interface().read_report().ok(); // ?
    }
}

fn key_press(keys: &[&dyn InputPin<Error = Infallible>]) -> [Keyboard; NBKEYS] {
    // ! put keys in a json, toml or something
    [
        //arrow UP:
        if keys[0].is_low().unwrap() {
            Keyboard::UpArrow
        } else {
            Keyboard::NoEventIndicated
        },
        //arrow LEFT:
        if keys[1].is_low().unwrap() {
            Keyboard::LeftArrow
        } else {
            Keyboard::NoEventIndicated
        },
        //arrow RIGHT:
        if keys[2].is_low().unwrap() {
            Keyboard::RightArrow
        } else {
            Keyboard::NoEventIndicated
        },
    ]
}

// End of file
