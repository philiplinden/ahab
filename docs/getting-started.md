# Getting Started

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
