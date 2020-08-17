# Protonvpn-rs

An UNOFFICIAL cli for protonvpn, based on the [official one (in python)](https://github.com/ProtonVPN/linux-cli).

## Motivation

The official cli is pretty great. It can be loads better in rust.

## Todo

- [x] CLI (using structopt)  
- [X] Config structs serialized to [ron](https://crates.io/crates/ron) with [serde](https://serde.rs/).
- [ ] `init` creates/checks for an existing login
- [ ] `configure` overwrites any individual settings
- [ ] openvpn connect/disconnect functions using the openvpn cli
- [ ] Bind connect/disconnect functions to protonvpn's cli
- [ ] `status` outputs server connection info
- [ ] Way more but the above is enough for now
