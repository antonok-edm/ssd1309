//! ssd1309 OLED display driver
//!
//! The driver must be initialised by passing a
//! [`display_interface`](https://crates.io/crates/display_interface) compatible interface
//! peripheral to the [`Builder`](builder/struct.Builder.html), which will in turn create a driver
//! instance in a particular mode. By default, the builder returns a `mode::RawMode` instance which
//! isn't very useful by itself. You can coerce the driver into a more useful mode by calling
//! `into()` and defining the type you want to coerce to. For example, to initialise the display
//! with an I2C interface and [`mode::GraphicsMode`](mode/graphics/struct.GraphicsMode.html), you
//! would do something like this:
//!
//! ```rust,ignore
//! let i2c = display_interface_i2c::I2CInterface::new(/* snip */);
//!
//! let mut disp: GraphicsMode<_> = Builder::new().connect(i2c).into();
//! disp.reset(/* snip */);
//! disp.init();
//! disp.set_pixel(10, 20, 1);
//! ```
//!
//! See the [example](https://github.com/antonok-edm/ssd1309/blob/master/examples/graphics.rs)
//! for more usage. The [entire `embedded_graphics` featureset](https://github.com/jamwaffles/embedded-graphics#features)
//! is supported by this driver.
//!
//! It's possible to customise the driver to suit your display/application. Take a look at the
//! [Builder] for available options.
//!
//! # Examples
//!
//! Examples can be found in
//! [the examples/ folder](https://github.com/antonok-edm/ssd1309/blob/master/examples)
//!
//! ## Draw some text to the display
//!
//! Uses [mode::GraphicsMode] and [embedded_graphics](../embedded_graphics/index.html).
//!
//! ```rust,no_run
//! #![no_std]
//! #![no_main]
//!
//! extern crate cortex_m;
//! extern crate embedded_graphics;
//! extern crate embedded_hal as hal;
//! extern crate panic_semihosting;
//! extern crate ssd1309;
//! extern crate stm32f1xx_hal as blue_pill;
//!
//! use blue_pill::pac::Peripherals;
//! use blue_pill::i2c::{DutyCycle, BlockingI2c, Mode};
//! use blue_pill::prelude::*;
//! use display_interface_i2c::I2CInterface;
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_5X8, MonoTextStyle},
//!        pixelcolor::BinaryColor,
//!        prelude::*,
//!        text::Text,
//!    };
//! use panic_semihosting as _;
//! use ssd1309::{mode::GraphicsMode, Builder};
//!
//! fn main() {
//!     let dp = blue_pill::pac::Peripherals::take().unwrap();
//!     let cp = cortex_m::Peripherals::take().unwrap();
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     let mut afio = dp.AFIO.constrain();
//!     let mut gpiob = dp.GPIOB.split();
//!     let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
//!     let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
//!     let mut res = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
//!     let mut delay = cp.SYST.delay(&clocks);
//!
//!     let i2c = BlockingI2c::i2c1(
//!         dp.I2C1,
//!         (scl, sda),
//!         &mut afio.mapr,
//!         Mode::Fast {
//!             frequency: 100u32.kHz(),
//!             duty_cycle: DutyCycle::Ratio2to1,
//!         },
//!         clocks,
//!         1000,
//!         10,
//!         1000,
//!         1000,
//!     );
//!
//!     let i2c_interface = I2CInterface::new(i2c, 0x3C, 0x40);
//!
//!     let mut disp: GraphicsMode<_> = Builder::new().connect(i2c_interface).into();
//!
//!     disp.reset(&mut res, &mut delay).unwrap();
//!     disp.init().unwrap();
//!     disp.flush().unwrap();
//! 
//!     let style = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);
//!
//!     Text::new("Hello world!", Point::new(0, 0), style).draw(&mut disp).unwrap();
//!     Text::new("Hello Rust!", Point::new(0, 16), style).draw(&mut disp).unwrap();
//!     disp.flush().unwrap();
//! }
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
mod displaysize;
pub mod mode;
pub mod prelude;
pub mod properties;

pub use crate::builder::{Builder, NoOutputPin};
