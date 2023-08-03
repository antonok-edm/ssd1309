//! Draw a 1 bit per pixel black and white image. On a 128x64 SSD1309 display over I2C.
//!
//! Image was created with ImageMagick:
//!
//! ```bash
//! convert rust.png -depth 1 gray:rust.raw
//! ```
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Wiring connections are as follows for a CRIUS-branded display:
//!
//! ```
//!      Display -> Blue Pill
//! (black)  GND -> GND
//! (red)    +5V -> VCC
//! (yellow) SDA -> PB9
//! (green)  SCL -> PB8
//! ```
//!
//! Run on a Blue Pill with `cargo run --example pixelsquare`.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use display_interface_i2c::I2CInterface;
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
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    let mut res = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let mut delay = cp.SYST.delay(&clocks);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100u32.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
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

    // Top side
    disp.set_pixel(0, 0, 1);
    disp.set_pixel(1, 0, 1);
    disp.set_pixel(2, 0, 1);
    disp.set_pixel(3, 0, 1);

    // Right side
    disp.set_pixel(3, 0, 1);
    disp.set_pixel(3, 1, 1);
    disp.set_pixel(3, 2, 1);
    disp.set_pixel(3, 3, 1);

    // Bottom side
    disp.set_pixel(0, 3, 1);
    disp.set_pixel(1, 3, 1);
    disp.set_pixel(2, 3, 1);
    disp.set_pixel(3, 3, 1);

    // Left side
    disp.set_pixel(0, 0, 1);
    disp.set_pixel(0, 1, 1);
    disp.set_pixel(0, 2, 1);
    disp.set_pixel(0, 3, 1);

    disp.flush().unwrap();

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
