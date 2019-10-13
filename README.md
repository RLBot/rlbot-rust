# <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" /> rlbot-rust

[![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
[![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
[![Build Status](https://travis-ci.org/rlbot/rlbot-rust.svg?branch=master)](https://travis-ci.org/rlbot/rlbot-rust)

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

RLBot is written in Python, so the easiest way to get the needed files is to use
`pip`:

```sh
pip install rlbot
```

Then add them to your `PATH` (adapt this command to the location of `rlbot` on
your particular system:

```sh
export PATH="$PATH":/c/Python36/Lib/site-packages/rlbot/dll
```

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
flatc -o src --rust rlbot.fbs
cargo fix --allow-dirty
cargo +nightly fmt
```

This will update the `src/rlbot_generated.rs` file.

[build flatc]: https://google.github.io/flatbuffers/flatbuffers_guide_building.html
[flatbuffer schema]: https://github.com/RLBot/RLBot/blob/master/src/main/flatbuffers/rlbot.fbs

### How to generate ffi bindings

Bindings are generated with [rust-bindgen]. Those docs are required reading.

[rust-bindgen]: https://rust-lang.github.io/rust-bindgen/

After bindgen and its prerequisites are installed and working, run this
delightfully short command:

```sh
rlbot=<absolute-path-to-rlbot>
bindgen \
    cpp/rlbot.hpp \
    -o src/ffi.rs \
    --disable-name-namespacing \
    --no-layout-tests \
    --default-enum-style rust \
    --with-derive-default \
    --raw-line '#![allow(non_camel_case_types, non_snake_case, missing_docs)]' \
    --whitelist-function BallPrediction::GetBallPrediction \
    --whitelist-function BallPrediction::GetBallPredictionStruct \
    --whitelist-function GameFunctions::Free \
    --whitelist-function GameFunctions::SetGameState \
    --whitelist-function GameFunctions::StartMatch \
    --whitelist-function GameFunctions::StartMatchFlatbuffer \
    --whitelist-function GameFunctions::UpdateFieldInfoFlatbuffer \
    --whitelist-function GameFunctions::UpdateFieldInfo \
    --whitelist-function GameFunctions::UpdateLiveDataPacketFlatbuffer \
    --whitelist-function GameFunctions::UpdateLiveDataPacket \
    --whitelist-function GameFunctions::UpdateRigidBodyTickFlatbuffer \
    --whitelist-function GameFunctions::UpdateRigidBodyTick \
    --whitelist-function GameFunctions::SendQuickChat \
    --whitelist-function GameFunctions::SendChat \
    --whitelist-function GameFunctions::UpdatePlayerInput \
    --whitelist-function GameFunctions::UpdatePlayerInputFlatbuffer \
    --whitelist-function Interface::IsInitialized \
    --whitelist-function RenderFunctions::RenderGroup \
    -- \
    -fdeclspec \
    -I "$rlbot"/src/main/cpp/RLBotInterface/RLBotInterface \
    -I "$rlbot"/src/main/cpp/RLBotInterface/RLBotMessages
```

It should output errors in white text. Modify RLBot's source to fix the errors.

If on an OS that uses forward slashes (ie not Windows), this can quickly
alleviate some of the pain, run from the `RLBotInterface` directory:

```sh
find . | xargs perl -pi -e 's/([\w\.])\\(\w)/$1\/$2/g'
```

Commenting out includes that may fail to resolve:

```sh
find . | xargs perl -pi -e 's/\#include \<Windows.h\>/\/\/<Windows.h>/g'
```

For any problematic references to boost, it will be easiest to just purge
indiscriminately. You may have to remove other things, like everything to do
with `MessageStorage`, `GameInput`, `RenderOutput` and `CallbackOutput`. Keep
running the above bindgen command and fixing errors (fun times!). After you've
subjected yourself to enough pain, it will run successfully and output more
errors, but in red text this time. As long as the errors are in red, that means
it worked!

Now open the resulting file (`src/ffi.rs`) and delete all the `extern "C" pub
fn` declarations at the end. Since the DLL is actually loaded using this
crate's `dll` module, there's no sense exposing the non-working functions—it
would just lead to confusion.
