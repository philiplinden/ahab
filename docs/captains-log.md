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

I should start using Linear for project management for this. I probably won't
stick to it very well, but it will be good practice.

I found some great rust CI examples in the
[actions-rs](https://github.com/actions-rs) repo. I'm going to use them to set
up a build workflow for this project.
