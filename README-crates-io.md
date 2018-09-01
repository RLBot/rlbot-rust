# rlbot-rust

[![pipeline status](https://gitlab.com/whatisaphone/rlbot-rust/badges/master/pipeline.svg)](https://gitlab.com/whatisaphone/rlbot-rust/commits/master)

<img src="https://github.com/RLBot/RLBot/raw/674a96b3330cd4de80eb50458dae97488723e187/images/RLBot.png" height="96" />

[RLBot] is a framework for creating offline Rocket League bots. This crate
exposes Rust bindings to RLBot's [RLBot_Core_Interface.dll]. It presents a
simple, safe interface that should feel comfortable to Rust developers.

[RLBot]: https://github.com/RLBot/RLBot
[RLBot_Core_Interface.dll]: https://github.com/RLBot/RLBot/tree/master/src/main/cpp/RLBotInterface

## Usage

```rust
let rlbot = rlbot::init()?;
rlbot.start_match(rlbot::MatchSettings::simple_1v1("Hero", "Villian"))?;

let mut packets = rlbot.packeteer();

// Wait for the match to start. `packets.next()` sleeps until the next
// packet is available, so this loop will not roast your CPU :)
while !packets.next()?.GameInfo.RoundActive {}

loop {
    let packet = packets.next()?;
    let input: rlbot::PlayerInput = Default::default();
    rlbot.update_player_input(input, 0)?;
}
```

### Quick start

This repo comes with a simple example to get you started. It's called `atba`,
which stands for Always Towards Ball Agent. To try it out, open Rocket League,
and then run the example like so:

```sh
git clone https://gitlab.com/whatisaphone/rlbot-rust
cd rlbot
cargo run --example atba
```

If you get an error, chances are you need to download the framework! Proceed to
the next section.

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
