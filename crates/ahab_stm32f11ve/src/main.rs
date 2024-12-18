#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rtt_init_print, rprintln};

/// The entry point of the program.
///
/// This must never return. We denote this with `-> !` and make sure it happens
/// by including an unbounded loop in the function.
///
/// Print "Hello, world!" on startup then print a dot every second.
#[entry]
fn main() -> ! {
    // Initialize RTT for printing
    rtt_init_print!();

    // Print a message to the console
    rprintln!("Hello, world!");

    loop {
        // print a dot every second
        rprintln!(".");
        for _ in 0..100_000 {
            nop();
        }
    }
}
