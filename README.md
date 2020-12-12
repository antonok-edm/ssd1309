# SSD1309 driver

[![Crates.io](https://img.shields.io/crates/v/ssd1309.svg)](https://crates.io/crates/ssd1309)
[![Docs.rs](https://docs.rs/ssd1309/badge.svg)](https://docs.rs/ssd1309)

[![SSD1309 display module showing the Rust logo](readme_banner.jpg?raw=true)](examples/image.rs)

I2C/SPI driver for the SSD1309 OLED display written in 100% Rust.

Heavily based off of the [`SH1106 driver`](https://github.com/jamwaffles/sh1106) by @jamwaffles.

## Implementation note

It's important to use correct reset logic for the SSD1309, unlike with some other display drivers.
The `GraphicsMode::reset` method is a good way to ensure this is accomplished.

## [Documentation](https://docs.rs/ssd1309)

From [`examples/text.rs`](examples/text.rs):

```rust
#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use display_interface_i2c::I2CInterface;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};
use panic_semihosting as _;
use ssd1309::{prelude::*, Builder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut res = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let mut delay = stm32f1xx_hal::delay::Delay::new(cp.SYST, clocks);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let i2c_interface = I2CInterface::new(i2c, 0x3C, 0x40);

    let mut disp: GraphicsMode<_> = Builder::new().connect(i2c_interface).into();

    disp.reset(&mut res, &mut delay).unwrap();

    disp.init().unwrap();
    disp.flush().unwrap();

    Text::new("Hello world!", Point::zero())
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut disp)
        .unwrap();

    Text::new("Hello Rust!", Point::new(0, 16))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
```

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
