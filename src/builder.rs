//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust,ignore
//! use display_interface_spi::SPIInterface;
//!
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! let spi_interface = SPIInterfaceNoCS::new(spi, dc);
//!
//! Builder::new().connect(spi_interface);
//! ```
//!
//! Connect over I2C, changing rotation
//!
//! ```rust,ignore
//! use display_interface_i2c::I2CInterface;
//!
//! let i2c = /* I2C interface from your HAL of choice */;
//!
//! let i2c_interface = I2CInterface::new(i2c, 0x3D, 0x40);
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .connect(i2c_interface);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`GraphicsMode` mode](../mode/graphics/struct.GraphicsMode.html):
//!
//! ```rust,ignore
//! let display: GraphicsMode<_> = Builder::new().connect(interface).into();
//! ```

use core::marker::PhantomData;
use hal::{self, digital::v2::OutputPin};

use crate::{
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    mode::{displaymode::DisplayMode, raw::RawMode},
    properties::DisplayProperties,
};

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder {
    display_size: DisplaySize,
    rotation: DisplayRotation,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new() -> Builder {
        Builder {
            display_size: DisplaySize::Display128x64,
            rotation: DisplayRotation::Rotate0,
        }
    }
}

impl Builder {
    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn with_size(self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..self
        }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation.
    pub fn with_rotation(self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..self }
    }

    /// Finish the builder and use the given interface to communicate with the display.
    pub fn connect<DI>(self, interface: DI) -> DisplayMode<RawMode<DI>>
    where
        DI: display_interface::WriteOnlyDataCommand,
    {
        let properties = DisplayProperties::new(
            interface,
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<DI>>::new(properties)
    }
}

/// Represents an unused output pin.
#[derive(Clone, Copy)]
pub struct NoOutputPin<PinE = ()> {
    _m: PhantomData<PinE>,
}

impl<PinE> NoOutputPin<PinE> {
    /// Create a new instance of `NoOutputPin`
    pub fn new() -> Self {
        Self { _m: PhantomData }
    }
}

impl<PinE> OutputPin for NoOutputPin<PinE> {
    type Error = PinE;
    fn set_low(&mut self) -> Result<(), PinE> {
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), PinE> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::NoOutputPin;
    use embedded_hal::digital::v2::OutputPin;

    enum SomeError {}

    struct SomeDriver<P: OutputPin<Error = SomeError>> {
        #[allow(dead_code)]
        p: P,
    }

    #[test]
    fn test_output_pin() {
        let p = NoOutputPin::new();
        let _d = SomeDriver { p };

        assert!(true);
    }
}
