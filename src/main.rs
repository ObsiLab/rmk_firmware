// main.rs file
// RMK_firmware
// @link https://github.com/ObsiLab/rmk_firmware
// Created by Lucas Placentino - 0bsilab

#![no_std]
// don't link the Rust standard library
#![no_main]
// see #[entry] below, from cortex_m_rt::entry

// ---- const : ----

const XTAL_FREQ_HZ: u32 = 12_000_000u32; // RPi Pico/RP2040 crystal freq
                                         // needed? :
const NBKEYS: usize = 3; // ! number of keys on the keyboard (80 ?), automatically get from keymap json or toml or other?

// ---- use : ----

use panic_halt as _;

// usbd hid
use usbd_human_interface_device::page::Keyboard;
//use usbd_human_interface_device::device::keyboard::{KeyboardLedsReport, NKROBootKeyboardInterface};
use embedded_hal as hal;
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use embedded_time::duration::Milliseconds;
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface;
use usbd_human_interface_device::prelude::*;
//? use embedded_time::rate::Hertz;
use core::convert::Infallible;
use hal::pac;
//use rp2040_hal as hal; // prefer embedded_hal ?
//?use hal::Clock;
use cortex_m_rt::entry;

// ? :
use embedded_time::clock::Error;
use embedded_time::duration::Fraction;
use embedded_time::Instant;
pub const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000u32);
use crate::hal::Timer; // only with rp2040_hal ?
                       // ? use hal::Timer;
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

// ---------------
/*

use log::info;

// The macro for our start-up function
use cortex_m_rt::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

//? Some traits we need (blinky sample code)
//use embedded_hal::digital::v2::InputPin;
//use embedded_hal::digital::v2::OutputPin;
//use embedded_hal::digital::v2::ToggleableOutputPin;
use embedded_time::fixed_point::FixedPoint;
use rp2040_hal::clocks::Clock;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;


/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
#[entry]
//fn main() -> () {
fn main() -> ! {
    // right one
    info!("Program start");

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let _clocks = hal::clocks::init_clocks_and_plls(
        //? _clocks or just clocks
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, _clocks.system_clock.freq().integer());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    loop { // loop ?
    // ...

    info!("printing Hello World");
    //println!("Hello, world!");
    }
}
*/

/*

===========================  TEST 2 :  ================================

*/

// * test
#[cfg(rp2040)]
use crate::mcu::rp2040::*;
//#[cfg_attr(predicate, attr)]
pub mod mcu;

const USBVID: u16 = 0x1209;
const USBPID: u16 = 0x0001;
// ! need to get PID from pid.codes (VID 0x1209)

///main function test 2
#[entry]
fn main() -> ! {
    // * test
    #[cfg(rp2040)]
    let test_mcu = mcu::rp2040::RP2040::new("RP2040", 8);

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

    let clock = TimerClock::new(&timer);

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_interface(
            NKROBootKeyboardInterface::default_config(&clock), // ! clock ?? clock=TimerClock:new(&timer) ??
        )
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(USBVID, USBPID)) // ! need to get PID from pid.codes (VID 0x1209)
        .manufacturer("0bsilab")
        .product("Quanta Keyboard")
        .serial_number("TESTv1")
        //.serial_number("MYVERYOWNQUANTAKEYBOARD")
        .supports_remote_wakeup(false) //? should test this
        .max_packet_size_0(8)
        .build();

    let mut led_pin = pins.gpio13.into_push_pull_output(); // ! check pin // used for caps lock led
    led_pin.set_low().ok();

    let keys: &[&dyn InputPin<Error = core::convert::Infallible>] = &[
        // ! check pins, length must be == NBKEYS, maybe autogenerate ?
        &pins.gpio1.into_pull_up_input(),
        &pins.gpio2.into_pull_up_input(),
        &pins.gpio3.into_pull_up_input(), //* etc
    ];

    let reset_button = pins.gpio0.into_pull_up_input(); // ! check pin

    let mut input_count_down = timer.count_down();
    input_count_down.start(Milliseconds(1)); // ! check and test 10ms ?

    let mut tick_count_down = timer.count_down();
    tick_count_down.start(Milliseconds(1)); // ! check ms

    loop {
        // ! check
        if reset_button.is_low().unwrap() {
            hal::rom_data::reset_to_usb_boot(0x1 << 13, 0x0);
        }

        //Poll the keys every millisecond
        if input_count_down.wait().is_ok() {
            let keys = key_press(keys);

            match keyboard.interface().write_report(&keys) {
                Err(UsbHidError::WouldBlock) => {}
                Err(UsbHidError::Duplicate) => {}
                Ok(_) => {}
                Err(error) => {
                    panic!("Keyboard report (write) error: {:?}", error)
                }
            };
        }

        if tick_count_down.wait().is_ok() {
            match keyboard.interface().tick() {
                Err(UsbHidError::WouldBlock) => {}
                Ok(_) => {}
                Err(error) => {
                    panic!("Keyboard tick error: {:?}", error)
                }
            };
        }

        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.interface().read_report() {
                // check if caps lock, etc, is on
                Err(UsbError::WouldBlock) => {
                    // blank
                }
                Err(error) => {
                    panic!("Keyboard report (read) error: {:?}", error)
                }
                Ok(leds) => {
                    led_pin.set_state(PinState::from(leds.caps_lock)).ok(); //turns on the caps lock LED
                }
            }
        }

        // needed? :
        log::logger().flush();
    }
}

// ! TODO ---------- create a Key Struct and implement the key_press fn below for it ----------
// ! TODO create a Key from Struct for each key that is in the keymap JSON/TOML/other

///key_press function, sends key that is pressed
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

/*

===========TEST 3=========

*/

/*
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::device::keyboard::{KeyboardLedsReport, NKROBootKeyboardInterface};
use usbd_human_interface_device::prelude::*;


let usb_alloc = UsbBusAllocator::new(usb_bus);

let mut keyboard = UsbHidClassBuilder::new()
    .add_interface(
        NKROBootKeyboardInterface::default_config(&clock),
    )
    .build(&usb_alloc);

let mut usb_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0x1209, 0x0001))
    .manufacturer("usbd-human-interface-device")
    .product("NKRO Keyboard")
    .serial_number("TEST")
    .build();

loop {

    let keys = if pin.is_high().unwrap() {
            &[Keyboard::A]
        } else {
            &[Keyboard::NoEventIndicated]
    };

    keyboard.interface().write_report(keys).ok();
    keyboard.interface().tick().unwrap();

    if usb_dev.poll(&mut [&mut keyboard]) {
        match keyboard.interface().read_report() {

            Ok(l) => {
                update_leds(l);
            }
            _ => {}
        }
    }
}
*/

// End of file
