//! A blinky-LED example application
//! This example uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate stellaris_launchpad;
extern crate alloc;
extern crate collections;
extern crate embedded_serial;

use core::fmt::Write;
use stellaris_launchpad::cpu::{gpio, systick, timer, uart};
use embedded_serial::{BlockingTx, NonBlockingRx};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************


#[no_mangle]
pub extern "C" fn main() {
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    let mut loops = 0;
    let mut ticks_last = systick::SYSTICK_MAX;
    let mut t = timer::Timer::new(timer::TimerId::Timer1A);
    t.enable_pwm(4096);
    gpio::set_direction(gpio::PinPort::PortF(gpio::Pin::Pin2),
                        gpio::PinMode::Peripheral);
    gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin2));
    let levels = [1u32, 256, 512, 1024, 2048, 4096];
    uart.puts("Welcome to Launchpad Blink\n").unwrap();
    loop {
        for level in levels.iter() {
            t.set_pwm(*level);
            let delta = systick::get_since(ticks_last);
            ticks_last = systick::get_ticks();
            writeln!(uart,
                     "Hello, world! Loops = {}, elapsed = {}, run_time = {}, level = {}",
                     loops,
                     systick::ticks_to_usecs(delta),
                     systick::run_time_us() as u32,
                     level)
                    .unwrap();
            while let Ok(Some(ch)) = uart.getc_try() {
                writeln!(uart, "byte read {}", ch).unwrap();
            }
            loops = loops + 1;
            stellaris_launchpad::delay(250);
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
