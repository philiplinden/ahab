# Debugging the STM32F11VE

Based on [this tutorial](https://youtu.be/TOAynddiu5M).

## With RTT

To debug the device, we can use "real time transfer", RTT. RTT is a debugging
protocol that allows you to send and receive data to the device from a host
computer over the debug interface.

Of course, there's a crate for this:
[rtt-target](https://crates.io/crates/rtt-target).
It requires a "critical section" feature, but we can get that from
[cortex-m](https://crates.io/crates/cortex-m). Then enable the RTT feature in
the `Embed.toml` file.

```toml
[default.rtt]
enabled = true
```

## With GDB (maybe not supported on Windows)

GDB is a bit more complex to set up than RTT, but it's more powerful.

The GDB toolchain is available on Windows, Linux, and macOS but the installation
depends on the operating system.
[https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)

Then set up the GDB configs in `Embed.toml`. Don't forget to disable RTT if
you're using GDB.

```toml
[default.rtt]
enabled = false

[default.gdb]
enabled = true

[default.reset]
# Wait for the debugger to connect before resetting the device
halt_afterwards = true
```

Flash the device with `cargo embed` as usual. After it flashes, it halts the
device until the GDB server is connected.

```sh
❯ cargo embed
      Config default
      Target E:\repos\philiplinden\ahab\target\thumbv7em-none-eabihf\debug\ahab_stm32f11ve
      Erasing ✔ 100% [####################]  16.00 KiB @  40.06 KiB/s (took 0s)
  Programming ✔ 100% [####################]   2.00 KiB @   4.12 KiB/s (took 0s)
     Finished in 0.49s
    GDB stub listening at 127.0.0.1:1337
```
In a separate terminal, run the GDB server.

```sh
❯ arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/ahab_stm32f11ve
GNU gdb (Arm GNU Toolchain 14.2.Rel1 (Build arm-14.52)) 15.2.90.20241130-git
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "--host=x86_64-w64-mingw32 --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.linaro.org/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
target/thumbv7em-none-eabihf/debug/ahab_stm32f11ve: No such file or directory.
(gdb)
```
And connect to the device. By default, the GDB server listens on port 1337. You
can include the local host address or just `:1337` to connect to the device.

```sh
(gdb) target remote 127.0.0.1:1337
Remote debugging using 127.0.0.1:1337
warning: No executable has been specified and target does not support
determining executable automatically.  Try using the "file" command.
0x080004e8 in ?? ()
```

Aaaaaand it doesn't work for me. This seems like
[an active issue](https://github.com/probe-rs/probe-rs/issues/1057) with
`probe-rs`, as indicated by the log message "GDB **stub** listening...".

Stick to RTT for now.
