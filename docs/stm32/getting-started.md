# Getting started with the STM32F411VE

Here are some notes on how to set up a development environment to work on this
project. YMMV.

I recommend following [this video tutorial](https://youtu.be/TOAynddiu5M) to get
started.

We'll use the
[STM32F411VE](https://www.st.com/resource/en/datasheet/stm32f411ve.pdf) as our
target system.

## Setting up your development environment

1. Install Rust 1.31 or newer.
2. Add cortex-m targets to your toolchain. We are using Cortex-M4F, so:
   `rustup target add thumbv7em-none-eabihf`
3. Read the [Embedded Rust Book](https://docs.rust-embedded.org/book) in its
   entirety. Just kidding, but it's a good idea to bookmark it.
4. If you're using VSCode or one of its derivatives, install the
   [rust-analyzer](https://rust-analyzer.github.io/) extension and the
   [Cortex-Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)
   extension. There are some more details
   [here](https://github.com/rust-embedded/cortex-m-quickstart/blob/master/.vscode/README.md#customizing-for-other-targets)
   for advanced configurations.

## Setting up the project

### Configuring the compiler target

A target triple is a string that describes the target architecture, vendor,
operating system, and environment.

```
<arch><subarch>-<vendor>-<sys>-<env>
```

- `arch` is the architecture, e.g. `arm`, `x86`, `aarch64`, etc.
- `subarch` is the subarchitecture, e.g. `v7`, `v8`, `v9`, etc.
- `vendor` is the vendor, e.g. `none`, `apple`, `nvidia`, `intel`, etc.
- `sys` is the operating system, e.g. `none`, `linux`, `windows`, `macos`, etc.
- `env` is the environment, e.g. `eabihf`, `gnu`, `musl`, etc.

Both the MSP432 and the STM32F411E use the
[Cortex-M4F](https://developer.arm.com/Processors/Cortex-M4) with a floating
point unit. This chip is part of the ARMv7e-M architecture family.

The [rustc docs](https://doc.rust-lang.org/rustc/platform-support.html) show
that the target triple for ARMv7E-M is `thumbv7em-none-eabi` or
`thumbv7em-none-eabihf`.

- The `thumbv7em` prefix indicates that our Cortex-M4F uses the Thumb-2
  instruction set and the ARMv7E-M architecture.
- The `none` vendor indicates that the target does not have an operating system.
- The `eabi` or `eabihf` suffix indicates the
  [ABI](https://en.wikipedia.org/wiki/Application_binary_interface) to use.
  Since we have an FPU, we use the `eabihf` variant.

Now that we know the target triple, we can add it to the cargo toolchain on our
host machine and in our `.cargo/config.toml` file to tell the compiler how to
build code for the project's target.

```sh
# add the target architecture to the toolchain
rustup target add thumbv7em-none-eabihf
```

And to make sure the correct target and flags are used every `cargo build`, we
can add the following to our `.cargo/config.toml` file so we don't have to type
it out every time.

```toml
[build]
target = "thumbv7em-none-eabihf"     # Cortex-M4F and Cortex-M7F (with FPU)

[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Wl,-Tlink.x"]
```

But the IDE might still complain about failing to build the project for the host
architecture. We can instruct the IDE extensions to only build for the embedded
target by adding some extra configs. The following is an example for VSCode.

```json
// .vscode/settings.json
{
    "rust-analyzer.cargo.allTargets": false,
    "rust-analyzer.cargo.target": "thumbv7em-none-eabihf"
}
```

### Configuring the Cortex-M crates

The embedded Rust tools make it easy to build and run programs on the target but
require a little extra setup to get running for our specific target.

The first crate to set up is the runtime crate, `cortex-m-rt`. This crate
provides the `entry!` macro, which is used to mark the entry point of the
program, and interrupt handling. The
[cortex-m-rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/) crate expects a
`memory.x` file that specifies the flash and RAM memory layouts of the target.

![stm32f411ve memory map](_assets/stm32f411ve-memory-map.png)

According to the datasheet, the flash memory is at `0x0800 0000 - 0x0807 FFFF`
and the RAM is at `0x2000 0000 - 0x2002 0000`.
