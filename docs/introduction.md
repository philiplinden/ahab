# Introduction

I want to build a small, low-cost, (low-power?) high altitude balloon that
"punches above its weight class" in terms of functionality and is grown from the
ground up in a Rust ecosystem. The ingredients for this kind of project have been
around for a while, and I the last time I tried to do this I got stuck in a few
places I'm ready to get past this time.

The previous project suffered from a lack of discipline, way too much
complexity, and honest-to-god fear of tools and libraries in Rust.

Here's what I'm thinking...

## Rust everywhere

I want to use Rust for the software and firmware. Embedded Rust is an
interesting area and evolving quickly. I want to learn it. I really enjoy
working in Rust and the Cargo toolchain is incredible.

The embedded Rust ecosystem is growing quickly, and I think it's a good idea to
use it. The higher level software (like what would run on a Raspberry Pi) can be
written in Rust, and more institutions are starting to use Rust for their code
so it's a good skill to have.

Other systems like Bevy and remote protocol or networking tools in Rust are
growing quickly as well and I think they are suited perfectly for the kinds of
needs I will have with this project. Using a game engine like Bevy for modeling
and simulation is somewhat novel but I think it's a good fit since it is also
Rust native and is fully featured.

## Everything in the loop

I want to develop the software and firmware with tests that can be run directly
on the hardware with everything in the loop. I want to be able to test the
software with "fake" inputs and outputs that react as I expect them to in real
life. The goal is to have everything working on my desk and then have everything
run exactly the same way when it's time for flight.

## Use what's in my drawer

We don't need to buy anything new or use anything custom. It would be nice,
sure, but that's one of the things that killed the last project.

Here's a list of what I found in my drawer (it's more than enough to get
started!):

- tiny gps breakout https://www.sparkfun.com/products/retired/10995
- rpi 4b
- rpi zero w 1.1
- msp432
- stm32F411e
- arduino mkr fpga https://www.arduino.cc/en/Guide/MKRVidor4000
- a TTL camera

We're missing power systems and radios, but I think there's plenty here to get
the fundamentals working and add those later. The systems can operate on the
bench with "shore power" and a data umbilical. Later on, both of those can be
replaced with battery and radio modules that are drop-in replacements. By making
this assumption, we pretty much guarantee that we'll understand the power
requirements and communication protocols with a high degree of confidence. Then
later, we part out those systems accordingly. Since we're not constrained by
power or comms in terms of the design, this is totally acceptable.
