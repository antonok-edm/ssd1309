//! ssd1309 Communication Interface (I2C/SPI)
//!
//! These types are re-exported from the `display_interface` family of crates.

/// A method of communicating with ssd1309
pub use display_interface::WriteOnlyDataCommand as DisplayInterface;

pub use display_interface_i2c::I2CInterface as I2cInterface;
pub use display_interface_spi::SPIInterface as SpiInterface;
