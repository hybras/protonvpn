# Protonvpn-rs

[![Crate Badge](https://buildstats.info/crate/protonvpn)](https://crates.io/crates/protonvpn)

An UNOFFICIAL cli for protonvpn, based on the [official one (in python)](https://github.com/ProtonVPN/linux-cli).

## Status

Currently all this crate is capable of is editing settings (and not very well). The password is not stored yet, and this was all done without using tui libs (I didn't know about them at the time).

## Links for Later

[CLI Libs](https://lib.rs/command-line-interface)
[`std::process`](https://doc.rust-lang.org/std/process/index.html)

## Todo

- [x] CLI (using structopt)  
- [X] Config structs serialized to [ron](https://crates.io/crates/ron) with [serde](https://serde.rs/).
- [X] `init` creates/checks for an existing login
- [X] `configure` overwrites any individual settings
- [ ] openvpn connect/disconnect functions using the openvpn cli
- [ ] Bind connect/disconnect functions to protonvpn's cli
- [ ] `status` outputs server connection info
- [ ] Way more but the above is enough for now

## License

Licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

at your option.
