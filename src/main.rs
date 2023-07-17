//! PWM based cotroller for treadmill motors using stock power supplies.
//! 
//! Uses a microcontroller with minimum or a Rotray encoder, a hall-effect 
//! sensor and a power switch.
//! 
//! Outputs a PWM signal with a duration of 51ms and changes the duty cycle
//! of the signal to tell the power supply how fast to spin the motor.
//! 
//! # Notes  
//! * Duration is seconds/cycle. Frequency is cycles/second. Therefore
//! duration is 1/freq and freq is 1/duration. The more you know!!! ;)
//! 
//! * Duty cycle is the percentage of 1 cycle in which the signal is high. 
//! 
//!     * E.G. if our 51ms cycle looks like -> HIGH for 30ms then LOW for 21ms 
//! our duty cycle is 30/51. 
//! 
//! * The way these power supplies work is increasing
//! the duty cycle increases the speed and decreasing the duty cycle 
//! decreases it.

#![no_std]
#![no_main]
#![deny(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]


// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
