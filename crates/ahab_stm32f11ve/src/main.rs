#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

/// The entry point of the program.
///
/// This must never return. We denote this with `-> !` and make sure it happens
/// by including an unbounded loop in the function.
#[entry]
fn main() -> ! {
    loop {
        // your code goes here
    }
}
