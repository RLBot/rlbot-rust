# <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" /> rlbot

[![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
[![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
[![Build Status](https://travis-ci.org/whatisaphone/rlbot-rust.svg?branch=master)](https://travis-ci.org/whatisaphone/rlbot-rust)

[RLBot] is a framework for creating offline Rocket League bots. This crate lets
you write bots using a simple, safe interface that should feel comfortable to
Rust developers.

**Stability:** As you might notice, we're still on version 0.x. Breaking changes
are likely at this stage. Join the [Discord] to keep up-to-date!

[RLBot]: https://github.com/RLBot/RLBot
[Discord]: https://discordapp.com/invite/XhrQGf

## Usage

Your code will look a little something like this:

```rust
use rlbot::ffi;

fn main() -> Result<(), Box<Error>> {
    rlbot::run_bot(MyBot { /* ... */ })
}

struct MyBot { /* ... */ }

impl rlbot::Bot for MyBot {
    fn tick(&mut self, packet: &ffi::LiveDataPacket) -> ffi::PlayerInput {
        // ...
    }
}
```

See [`examples/bot`] for a complete example.

[`examples/bot`]: https://github.com/whatisaphone/rlbot-rust/blob/master/examples/bot/main.rs

### Quick start

This repo comes with a few examples to get you started.

#### `examples/simple`

This is a simple ATBA, or Always Towards Ball Agent. It can run with no
dependencies other than RLBot itself. You can run it like this:

```sh
cargo run --example simple
```

If you get an error, chances are you need to download the framework! Follow the
instructions under **Installing the framework**.

#### `examples/simple_flatbuffer`

Another ATBA, but using a secondary interface which uses flatbuffers. Many
functions in RLBot's core interface require flatbuffers.

```sh
cargo run --example simple_flatbuffer
```

#### `examples/rendering`

Shows how to draw simple shapes to the game window. If you don't see anything,
try pressing PageUp, which is RLBot's shortcut for turning on rendering.

```sh
cargo run --example rendering
```

#### `examples/gravity`

A fun example showing how to set game state using the low-level interface.

```sh
cargo run --example gravity
```

#### `examples/bot`

This is a full-fledged bot that can run within the Python RLBot framework. It
requires a working RLBot Python setup. Follow the instructions in
[RLBotPythonExample] to make sure you have all the necessary dependencies
installed. Once you have that working, you should be able to run a Rust bot
within the framework with this command:

```sh
cargo build --example bot && python -c "from rlbot import runner; runner.main()"
```

[RLBotPythonExample]: https://github.com/RLBot/RLBotPythonExample

### Installing the framework

RLBot is needed to use this RLBot binding, of course. If the framework is not
found in any of Windows's [DLL search locations], `init()` will return this
error:

[DLL search locations]: https://docs.microsoft.com/en-us/windows/desktop/dlls/dynamic-link-library-search-order#standard-search-order-for-desktop-applications

```text
Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
```

You'll need to download [these files] from RLBot:

[these files]: https://github.com/RLBot/RLBot/tree/master/src/main/python/rlbot/dll

* `RLBot_Injector.exe`
* `RLBot_Core.dll`
* `RLBot_Core_Interface.dll`

Place them in a directory in your `$PATH`. Alternatively, if you don't want to
pollute your system, place them in your crate's target directory, e.g.
`target/debug` or `target/release`).
