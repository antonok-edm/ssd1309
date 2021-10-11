//! Buffered display module for use with the [embedded_graphics] crate
//!
//! ```rust,ignore
//! let interface = /* your preferred `display-interface` implementor */;
//! let display: GraphicsMode<_> = Builder::new().connect(interface).into();
//! let image = include_bytes!("image_16x16.raw");
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//! display.draw(Line::new(Coord::new(0, 0), (16, 16), 1.into()).into_iter());
//! display.draw(Rect::new(Coord::new(24, 0), (40, 16), 1u8.into()).into_iter());
//! display.draw(Circle::new(Coord::new(64, 8), 8, 1u8.into()).into_iter());
//! display.draw(Image1BPP::new(image, 0, 24));
//! display.draw(Font6x8::render_str("Hello Rust!", 1u8.into()).translate(Coord::new(24, 24)).into_iter());
//! display.flush().unwrap();
//! ```

use display_interface::{DisplayError, WriteOnlyDataCommand};
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    displayrotation::DisplayRotation, mode::displaymode::DisplayModeTrait,
    properties::DisplayProperties,
};

const BUFFER_SIZE: usize = 128 * 64 / 8;

/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create new GraphicsMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.buffer = [0; BUFFER_SIZE];
    }

    /// Reset display. This is very important on the SSD1309!
    ///
    /// This should be called before `init` or any other methods.
    pub fn reset<RST, DELAY, PinE>(&mut self, rst: &mut RST, delay: &mut DELAY) -> Result<(), PinE>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high()?;
        delay.delay_ms(10);
        rst.set_low()?;
        delay.delay_ms(10);
        rst.set_high()?;
        delay.delay_ms(10);
        Ok(())
    }

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        let display_size = self.properties.get_size();

        // Ensure the display buffer is at the origin of the display before we send the full frame
        // to prevent accidental offsets
        let (display_width, display_height) = display_size.dimensions();
        let column_offset = display_size.column_offset();
        self.properties.set_draw_area(
            (column_offset, 0),
            (display_width + column_offset, display_height),
        )?;

        let length = (display_width as usize) * (display_height as usize) / 8;

        self.properties.draw(&self.buffer[..length])
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        let idx = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                if x >= display_width as u32 {
                    return;
                }
                ((y as usize) / 8 * display_width as usize) + (x as usize)
            }

            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                if y >= display_width as u32 {
                    return;
                }
                ((x as usize) / 8 * display_width as usize) + (y as usize)
            }
        };

        if idx >= self.buffer.len() {
            return;
        }

        let (byte, bit) = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let byte =
                    &mut self.buffer[((y as usize) / 8 * display_width as usize) + (x as usize)];
                let bit = 1 << (y % 8);

                (byte, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let byte =
                    &mut self.buffer[((x as usize) / 8 * display_width as usize) + (y as usize)];
                let bit = 1 << (x % 8);

                (byte, bit)
            }
        };

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.properties.init_column_mode()
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.properties.get_dimensions()
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.properties.set_rotation(rot)
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.properties.display_on(on)
    }

    /// Set the display contrast
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.properties.set_contrast(contrast)
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU1},
        BinaryColor,
    },
    prelude::*,
};

#[cfg(feature = "graphics")]
impl<DI> DrawTarget for GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(pos, color) in pixels.into_iter() {
            // Guard against negative values. All positive i32 values from `pos` can be represented in
            // the `u32`s that `set_pixel()` accepts...
            if pos.x >= 0 && pos.y >= 0 {
                self.set_pixel(pos.x as u32, pos.y as u32, RawU1::from(color).into_inner());
            }
        }

        Ok(())
    }
}

#[cfg(feature = "graphics")]
impl<DI> OriginDimensions for GraphicsMode<DI>
where
    DI: WriteOnlyDataCommand,
{
    fn size(&self) -> Size {
        let (w, h) = self.get_dimensions();

        Size::new(w as u32, h as u32)
    }
}
