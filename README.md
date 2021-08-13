# Rust PulseAudio API

[![docs](https://docs.rs/pulsectl-rs/badge.svg)](https://docs.rs/pulsectl-rs/) [![crates.io](https://img.shields.io/crates/v/pulsectl-rs)](https://crates.io/crates/pulsectl-rs) ![CI](https://github.com/SeaDve/pulsectl-rs/actions/workflows/ci.yml/badge.svg)

`pulsectl-rs` is a API wrapper for `libpulse_binding` to make pulseaudio
application development easier. This is a wrapper around the introspector, and
thus this library is only capable of modifying PulseAudio data (changing volume,
routing applications and muting right now).

This is a fork of [`pulsectl-rust-fork`](https://github.com/JojiiOfficial/pulsectl) 
which is then a fork of [`pulsectl-rust`](https://github.com/krruzic/pulsectl).

### Usage

Connect to PulseAudio by creating a `SinkController` for audio playback devices
and apps or a `SourceController` for audio recording devices and apps.

```rust
// Simple application that lists all playback devices and their status
// See examples/change_device_vol.rs for a more complete example
extern crate pulsectl;

use std::io;

use pulsectl::controllers::SinkController;
use pulsectl::controllers::DeviceControl;
fn main() {
    // create handler that calls functions on playback devices and apps
    let mut handler = SinkController::create();
    let devices = handler
        .list_devices()
        .expect("Could not get list of playback devices");
    println!("Playback Devices");
    for dev in devices.clone() {
        println!(
            "[{}] {}, [Volume: {}]",
            dev.index,
            dev.description.as_ref().unwrap(),
            dev.volume.print()
        );
    }
}
```
