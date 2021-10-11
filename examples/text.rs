//! Print "Hello world!" with "Hello rust!" underneath. Uses the `embedded_graphics` crate to draw
//! the text with a 5x8 pixel font.
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
//! Run on a Blue Pill with `cargo run --example text`.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use display_interface_i2c::I2CInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8 as Font, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
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

    let style = MonoTextStyle::new(&Font, BinaryColor::On);

    Text::with_baseline("Hello world!", Point::zero(), style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
