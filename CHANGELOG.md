# Changelog

## v0.4.2 – 2019-04-21

### Changed

* Updated to work with the latest RLBot.

## v0.4.1 – 2019-04-14

### Changed

* Updated dependencies to address [this
  issue](https://github.com/rustsim/ncollide/issues/266).

## v0.4.0 – 2019-03-10

### Added

* High-level `start_match`, `update_player_input`, and `GameTickPacket`.
* Convenience method for `rlbot_vs_rlbot` matches.
* `Physicist::next_flat_with_timeout`, which lets the caller specify the
  timeout.

### Changed

* Updated to Rust 2018.
* Updated to the latest RLBot version.
* The low-level interface is now behind a method (`RLBot::interface()`) instead
  of a field.

### Deprecated

* FFI methods/structs have been mass-deprecated, since RLBot considers them to
  be an implementation detail and does not provide any stability guarantees. Use
  flatbuffers instead.

## v0.3.0

### Added

* Load DLLs from their framework location, if given. ([@whatisaphone])
* An example showing off quick-chat with FlatBuffers. ([@whatisaphone])

### Changed

* Deprecated `MatchSettings::simple_1v1` in favor of functions with more
  descriptive names (`rlbot_vs_allstar` and `allstar_vs_allstar`).
  ([@whatisaphone])
* Wait longer in `Physicist` and `Packeteer` before giving up and returning an
  error. ([@whatisaphone])

### Fixed

* Updated to the latest framework version. ([@whatisaphone])

## v0.2.0 – 2018-12-02

### Added

* High-level state setting interface. ([@whatisaphone])
* `wait_for_match_start` function. ([@whatisaphone])

## v0.1.2 – 2018-11-30

### Added

* Physicist gained `next` and `try_next`. Packeteer gained `try_next` and
  `try_next_flat`. This brings the two objects to feature parity with each
  other. ([@whatisaphone])

### Changed

* Loosened the unnecessarily restrictive lifetimes for flatbuffers.
  ([@whatisaphone])

## v0.1.1 – 2018-10-28

### Changed

* Upgraded `flatbuffers`. ([@whatisaphone])
* Bumped the minimum supported Rust version to 1.28. ([@whatisaphone])

## v0.1.0 – 2018-10-10

### Added

* Support for line rendering. ([@whatisaphone])
* Support for RigidBodyTick. ([@whatisaphone])
* [internal] More tests. ([@whatisaphone])

### Fixed

* Compatibility with latest RLBot. ([@whatisaphone])

## v0.0.6 – 2018-09-25

### Added

* Wrappers for all remaining RLBot interface functions. ([@ehsanul])
* Example for state setting. ([@ehsanul])

## v0.0.5 – 2018-09-13

### Added

* FlatBuffer support. ([@ehsanul])

### Changed

* Migrated from GitLab to GitHub. ([@whatisaphone])
* Migrated from GitLab CI to Travis CI. ([@whatisaphone])

# Contributors

Thanks!

* [@whatisaphone](https://github.com/whatisaphone) since v0.0.1
* [@ehsanul](https://github.com/ehsanul) since v0.0.5

[@whatisaphone]: https://github.com/whatisaphone
[@ehsanul]: https://github.com/ehsanul
