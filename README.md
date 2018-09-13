# <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" /> rlbot-rust <img src="https://github.com/RLBot/RLBot/raw/674a96b3330cd4de80eb50458dae97488723e187/images/RLBot.png" height="96" align="right" />

[![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
[![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
[![Build Status](https://travis-ci.org/whatisaphone/rlbot-rust.svg?branch=master)](https://travis-ci.org/whatisaphone/rlbot-rust)

[RLBot] is a framework for creating offline Rocket League bots. This crate lets
you write bots using a simple, safe interface that should feel comfortable to
Rust developers.

**Stability:** As you might notice, we're still on version 0.0.x. Breaking
changes are likely at this stage. Join the [Discord] to keep up-to-date!

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

## Development

### Prerequisites

* Use [rustup] to install both stable Rust and nightly Rust.

  This library targets stable Rust, however nightly Rust is needed for a few
  development niceties like `cargo fmt` and `cargo doc`.

* Install [pre-commit], and run this command:

  ```sh
  pre-commit install
  ```

  This makes it much less likely that your commits will break the build.

[rustup]: https://rustup.rs/
[pre-commit]: https://pre-commit.com/

### Testing

Many of the tests require a copy of Rocket League. All such tests will have
`integration` in their name. Because these tests cannot be run in CI, they
should be run manually before cutting a release, using this command:

```sh
cargo test -- integration
```

### How to compile the flatbuffer schema

Flatbuffers comes with a schema compiler, flatc. Unless your package manager
has flatc and allows building HEAD, you'll have to [build flatc] yourself.

Get the most recent [flatbuffer schema]. Then compile the schema like so from
this project's root:

```sh
flatc -o src --rust rlbot.fbs && cargo fmt
```

This will update the `src/rlbot_generated.rs` file. One manual addition that
must be made is adding the following under the `rlbot` module defined within:

```rust
#![allow(non_camel_case_types, non_snake_case, missing_docs)]
```

[build flatc]: https://google.github.io/flatbuffers/flatbuffers_guide_building.html
[flatbuffer schema]: https://github.com/RLBot/RLBot/blob/master/src/main/flatbuffers/rlbot.fbs

### How to generate ffi bindings

Bindings are generated with [rust-bindgen]. Those docs are required reading.

[rust-bindgen]: https://rust-lang-nursery.github.io/rust-bindgen/

After bindgen and its prerequisites are installed and working, run this
delightfully short command:

```sh
rlbot=<path-to-rlbot>
bindgen \
    "$rlbot"/src/main/cpp/RLBotInterface/RLBotInterface/Interface.hpp \
    -o src/ffi.rs \
    --disable-name-namespacing \
    --no-layout-tests \
    --default-enum-style rust \
    --with-derive-default \
    --raw-line '#![allow(non_camel_case_types, non_snake_case, missing_docs)]' \
    --whitelist-function Interface::IsInitialized \
    --whitelist-function GameFunctions::SetGameState \
    --whitelist-function GameFunctions::StartMatch \
    --whitelist-function GameFunctions::UpdateFieldInfo \
    --whitelist-function GameFunctions::UpdateLiveDataPacket \
    --whitelist-function GameFunctions::UpdateLiveDataPacketFlatbuffer \
    --whitelist-function GameFunctions::SendQuickChat \
    --whitelist-function GameFunctions::SendChat \
    --whitelist-function GameFunctions::UpdatePlayerInput \
    --whitelist-function GameFunctions::UpdatePlayerInputFlatbuffer \
    --whitelist-function RenderFunctions::RenderGroup \
    -- \
    -I "$rlbot"/src/main/cpp/RLBotInterface/RLBotMessages
```

It should output errors in white text. Modify RLBot's source to fix the errors.
For any problematic references to boost, it will be easiest to just purge
indiscriminately. Keep running the above command and fixing errors (fun times!).
After you've subjected yourself to enough pain, it will run successfully and
output more errors, but in red text this time. As long as the errors are in red,
that means it worked!

Now open the resulting file (`src/ffi.rs`) and delete all the `extern "C" pub
fn` declarations at the end. Since the DLL is actually loaded using this
crate's `dll` module, there's no sense exposing the non-working functions—it
would just lead to confusion.
