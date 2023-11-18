# SSD1309 driver

[![Crates.io](https://img.shields.io/crates/v/ssd1309.svg)](https://crates.io/crates/ssd1309)
[![Docs.rs](https://docs.rs/ssd1309/badge.svg)](https://docs.rs/ssd1309)

[![SSD1309 display module showing the Rust logo](readme_banner.jpg?raw=true)](examples/image.rs)

I2C/SPI driver for the SSD1309 OLED display written in 100% Rust.

Heavily based off of the [`SH1106 driver`](https://github.com/jamwaffles/sh1106) by @jamwaffles.

## Implementation note

It's important to use correct reset logic for the SSD1309, unlike with some other display drivers.
The `GraphicsMode::reset` method is a good way to ensure this is accomplished.

## Usage

Check the [documentation](https://docs.rs/ssd1309) and [examples](examples/).

Also available are third-party examples using the [Cortex-M4F LaunchPad (TM4C123G)](https://github.com/HerrMuellerluedenscheid/tm4c-oled-example) and the [Raspberry Pi Pico (RP2040)](https://github.com/HerrMuellerluedenscheid/rp2040-oled-1309-spi).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
