# captain's log

## 2024-12-21

Renamed the `stm32` directory to `hello-world`.

I did some more thinking about the performance considerations of the RPi Zero,
MSP432, and STM32F411xx. The MSP432 is the most power-efficient at idle, but I'm
not expecting to operate the balloon for weeks at idle. The STM32 board is a bit
faster and more power-efficient than the RPi Zero, but not as efficient as the
MSP432. The RPi Zero is the fastest and least power-efficient of the three, but
it is far more flexible and capable than the other two. If I want to do imaging,
the RPi Zero is the only realistic option due to compute and memory. If I leave
out imaging, the STM32 is plenty. The dev board for the STM32F411VE comes with
handy sensors that will be useful to me too. In the end, I think I'll use the
STM32F411VE for flight and the RPi Zero for ground station.

I'm not sure how radio factors into this yet. Maybe the RPi Zero will be
necessary for radio and I can fly them as a pair. I have a full size Pi 4B
sitting around that I could use for ground station.

The [Nucleus](https://github.com/Brickworks/Nucleus) HAB used a Pi 4 compute
module as the main computer, so using a Pi for flight might be a good idea if I
want to continue that project someday.

Turns out I have the MB1115B variant, which uses
- [L3GD20 3-axis gyroscope](../crates/ahab_stm32f11ve/docs/l3gd20.pdf)
- [LSM303DLHC e-compass](../crates/ahab_stm32f11ve/docs/lsm303dlhc.pdf)
- [MP45DT02 digital MEMS microphone](../crates/ahab_stm32f11ve/docs/mp45dt02.pdf)

## 2024-12-20

[down-detector](hello-world/down-detector.md).


## 2024-12-19

Today I'll try to [blinky](https://www.youtube.com/watch?v=A9wvA_S6m7Y):
[blinky](hello-worldblinky.md). After that I'll try to do it again with embassy.

From the
[example](https://github.com/embassy-rs/embassy/tree/main/examples/stm32f4), it
looks like `embassy` doesn't replace the `cortex-m` crates, it just adds
convenience functions and ergonomics to it.

I stole some other options from the example's `Cargo.toml` file too like
`panic-probe` and `defmt`.

> Panic-Probe is a Rust crate that enables `probe-run`, a tool for running
> embedded Rust applications like native applications. It provides a panic
> handler for `probe-run`, allowing you to write integration tests that run on
> your embedded device via `cargo test`.

and

> `defmt` ("de format", short for "deferred formatting") is a highly efficient
> logging framework that targets resource-constrained devices, like
> microcontrollers. `println!`-like formatting, multiple logging levels,
> compile-time `RUST_LOG`-like filtering of logs, and timestamped logs.

When compiling, I'm finding that something about `embassy-time` is causing the
build to fail. I can successfully compile and build an embassy script without
`embassy-time` but any time I import a timer, it fails to compile. I might be
able to get around this by using `use embassy::time::{Duration, Timer};`.

Hm, a search engine AI says
> Undefined symbol _embassy_time_schedule_wake: The linker error occurs when
> trying to link the embassy_time crate with the generic-queue feature enabled.
> The error message indicates that there are multiple definitions of the
> _embassy_time_schedule_wake symbol.
> [Github Issue](https://github.com/embassy-rs/embassy/issues/1109)

And in that github issue I think I found some breadcrumbs toward a
[solution](https://github.com/embassy-rs/embassy/issues/1109#issuecomment-1346256031).

> To use timers, some backend must be provided through crate features –
> otherwise, linking will fail and complain about a missing
> `_embassy_time_schedule_wake` symbol.
> 
> Possible backends are:
> - The `integrated-timers` feature of the `embassy-executor` crate. If you use
>   this and no other executor, this is preferred.
> - (Something for ESP?)
> - The `generic-queue` feature of the embassy-time crate. Use this if you use
>   multiple executors, or one not listed above.

This didn't work alone, but I was able to get it to build once I added the
`time-driver-tim4` and `time` features to the `embassy-stm32` crate alongside
the `integrated-timers` feature in `embassy-executor`.

I ran into another issue when trying to reflash the board.

It looks very similar to the issue described
[here](https://users.rust-lang.org/t/probe-rs-fails-to-work-after-first-time-use-successful/103234/4)
which was resolved by adding `--connect-under-reset` to the `probe-rs` command.
After adding this option, flashing has been reliably working.

I also confirmed that embassy with the `memory-x` feature does indeed let us
delete the `memory.x` file from our repo and ignore it from the build process!

## 2024-12-18

There's a rust project that claims to support raspberry pi and stm32 boards,
[embassy.dev](https://docs.embassy.dev/embassy-stm32/git/stm32f411ve/index.html)
that looks interesting. I'm not sure how it differs from the `cortex-m` crates
I've been using, but it might be worth checking out.

- It looks like Embassy can auto-generate the `memory.x` file for the board.
- The docs are really good and filtered by the specific chipset.
- Doesn't look like it supports the MSP432.

I also did some shopping for parts, but I didn't order anything yet. Here are
the parts I'm considering:

- [RFM96W LoRa Transceiver (915 MHz)](https://www.sparkfun.com/products/12775).
  This also comes in a 433 MHz version, but as far as I can tell it's a region
  lock thing---The Americas use 915 Mhz for the license-free ISM band. The 915
  MHz version has an operating power of up to 100 mW and 300 kbps, which should
  be enough for me.
- [NEO-M9N GPS](https://www.sparkfun.com/products/18378). According to
  the datasheet it should work up to 80 km. This is the newest version of this
  chip but I think modules as old as NEO-M6 would work.
- [BME280 Atmospheric Sensor](https://www.sparkfun.com/products/15440). The
  myth, the legend, the classic pressure, humidity, and temperature sensor.

Some other parts that are overkill but would be nice to have:

- [Qwiic MicroPressure Sensor](https://www.sparkfun.com/products/16476). This is
  a barometer that has a calibrated sensing range of 60mbar to 2.5bar. I think
  it would be useful to have a sensitive barometer for higher altitudes where a
  more general purpose barometer might not be sensitive enough.
- [OpenLog Artemis](https://www.sparkfun.com/products/16832). This is a
  data logger board with a built-in IMU, voltage loggers, some high-rate
  sampling for a few channels or ~250Hz logging in general. It automatically
  detects, configures, and logs Qwiic boards, including all the sensors I'd want
  to use for this project.
- [Waveshare Environment Sensor HAT](https://www.waveshare.com/environment-sensor-hat.htm).
  This is basically a glorified BME280 carrier board with a few other sensors
  like UV, air quality, and convenient Raspberry Pi GPIO headers.
- [BerryGPS-IMU](https://ozzmaker.com/product/berrygps-imu/). A fancy all-in-one
  motion sensor designed to fit perfectly with the Raspberry Pi Pico. Fancy.
  Overkill. Cool, though. It has a GPS, 10DOF IMU, barometric altimeter,
  magnetometer, and temperature sensor. This board uses the CAM-M8 GPS module
  from u-blox, which dies at 50 km altitude.

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

And now, it's time to [dive in to embedded rust](https://youtu.be/TOAynddiu5M).

I deleted most of the template code and started from scratch following this
tutorial and documenting the process in
[docs/hello-worldgetting-started.md](docs/hello-worldgetting-started.md). I can always
re-use the template code if I need it, but building up from scratch is great
practice to learn what all of these files do and how they fit together.

Woohoo! I got a (dummy) program to compile for the STM32F411E board! Now to
flash it... Nice, it worked!

There is one hang up though. When I `cargo build` from the root, it tries to
compile the embedded crates for the host arch. I want to edit `Cargo.toml` at
the root to effectively do the same thing as specifying the target arch in
`.cargo/config.toml` for each crate.

https://doc.rust-lang.org/cargo/reference/unstable.html#per-package-target

https://nnarain.github.io/2021/08/02/Setting-up-per-package-targets-in-Rust.html

I had to add `cargo-features = ["per-package-target"]` to the `Cargo.toml` file,
which is an unstable `nightly` feature.
