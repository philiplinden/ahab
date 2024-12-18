# captain's log

## 2024-12-17

Ahab is a fitting name for this project. It is a high altitude balloon. It is
cheeky. It is a bit of a jerk. It is a bit of a showoff. It is a bit of a thing
I keep chasing and chasing and chasing. It's also a nod to the "alumni hab"
brickworks project that became [Nucleus](https://github.com/Brickworks/Nucleus).

I'm going to try to keep a log of my thoughts and ideas here, like a captain
losing his mind at sea.

Some things I have floating around in my head:

1. I really like mdBook. I think it's a great way to write documentation.
2. I found the following things in my drawer.
   - tiny gps breakout https://www.sparkfun.com/products/retired/10995
   - rpi 4b
   - rpi zero w 1.1
   - msp430
   - stm32F411e
   - arduino mkr fpga https://www.arduino.cc/en/Guide/MKRVidor4000
   - a TTL camera
3. I would really like to build out [yahs](https://github.com/philiplinden/yahs)
   into a thing that can be used to build and test a high altitude balloon,
   more than just a thing I use to learn rust.
4. I want to explore embedded rust.
5. I want to finish what [Nucleus](https://github.com/Brickworks/Nucleus)
   started. We had a lot of good ideas and made a lot of mistakes. I want to
   finish what we started.

I set up this repository and started writing some documentation, including some
notes on what I want out of this project.

I flashed the Raspberry Pi 4B with Raspberry Pi OS Lite and was able to
successfully SSH into it on my local network. Even if I end up using the Pi
Zero for flight, the 4B is really easy to set up on the network, has plenty of
headroom, and will serve as a good testbed for now. It could also host other
development services like a web server, database, Jenkins, etc if I need it to.

https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

I should start using Linear for project management for this. I probably won't
stick to it very well, but it will be good practice.

I found some great rust CI examples in the
[actions-rs](https://github.com/actions-rs) repo. I'm going to use them to set
up a build workflow for this project.

I set up a template for the STM32F411E board.
https://studiofuga.com/blog/2022-10-18-getting-started-with-rust-and-stm32f411/

I set up a template for the MSP432 too.
- https://github.com/rust-embedded/msp430-quickstart (not this board, but close)
- https://docs.rs-online.com/3934/A700000006811369.pdf
- https://github.com/MSP432P401R-Launchpad-Rust
- https://github.com/msp432-rust/msp432p401r-hal
- https://github.com/msp432-rust/msp432p401r-pac
- https://github.com/pcein/msp432p401r
- https://github.com/pcein/msp432-newio

Turns out the MSP432 uses the TM4C129 chip, which can be used with the
[cortex-m-quickstart](https://docs.rs/cortex-m-quickstart/~0.2.3) cargo-generate
template. I set that up but I'm not sure if the `memory.x` file is correct.

I ended up fighting the compiler for a while, trying to get the panic handler
to work. I ended up just using the `panic-semihosting` crate for now because
that's what would compile. Using `panic-halt` or `panic-abort` required using
deprecated features I guess? After switching to the `panic-semihosting` crate, I
started running into linker errors. I'll have to figure this out later once I
am actually flashing the boards with code I understand, rather than templates.

The root cause looks to be related to this comment
[from the miri repo](https://github.com/rust-lang/miri/issues/3498):
> you added a crate as dependency that depends on std, and built a sysroot
> without std, and then that fails to build. No surprise here.

Oh well.

I thought some more about how to structure the project so that I can swap out
blocks of logic easily between simulated code on my PC, code running on a board
in a hardware-in-the-loop simulation, and actual code running on the board in
the real world. With Rust, hopefully we can write tests and benchmarks for the
software and firmware so that they behave exactly the same way in each case. The
tricky bit is how to handle the interface between the simulated environments and
the real environments for things like sensors and actuators.

--> [design/structure.md](design/structure.md)
