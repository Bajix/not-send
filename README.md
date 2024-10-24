![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/not-send.svg)](https://crates.io/crates/not-send)
[![Documentation](https://docs.rs/not-send/badge.svg)](https://docs.rs/not-send)

Adding `not_send::PhantomNotSend` to types prevents `Send` from being automatically implemented.

Use the `#[serde(skip)]` for usage in types that are `Serialize`, `Deserialize`.