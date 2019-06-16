# <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" /> rlbot

[![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
[![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
[![Build Status](https://travis-ci.org/whatisaphone/rlbot-rust.svg?branch=master)](https://travis-ci.org/whatisaphone/rlbot-rust)

[RLBot] is a framework for creating offline Rocket League bots. This crate lets
you write bots using a simple, safe interface that should feel comfortable to
Rust developers.

**Documentation:** [We have it.](https://docs.rs/rlbot/)

**Stability:** As you might notice, we're still on version 0.x. Breaking changes
are still possible at this stage. Join the [Discord] to keep up-to-date!

**Compatibility**: We target the latest version of RLBot, and the latest stable
version of Rust.

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

This library comes with plenty of examples to get you started. For a list of
examples, check out the [docs].

[docs]: https://docs.rs/rlbot/

### Installing the framework

RLBot is needed to use this RLBot binding, of course. If the framework is not
found in any of Windows's [DLL search locations], `init()` will return this
error:

[DLL search locations]: https://docs.microsoft.com/en-us/windows/desktop/dlls/dynamic-link-library-search-order#standard-search-order-for-desktop-applications

```text
Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
```

RLBot is written in Python, so you can get a copy with `pip`:

```sh
pip install rlbot
```

Then add RLBot's DLL directory to your `PATH`. On my system it ended up here:

```sh
C:\Python36\Lib\site-packages\rlbot\dll
```
