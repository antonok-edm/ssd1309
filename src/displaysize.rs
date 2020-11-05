//! Display size

/// Display size enumeration
#[derive(Clone, Copy)]
pub enum DisplaySize {
    /// 128 by 64 pixels
    Display128x64,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    pub fn dimensions(self) -> (u8, u8) {
        match self {
            DisplaySize::Display128x64 => (128, 64),
        }
    }

    /// Get the panel column offset from DisplaySize
    pub fn column_offset(self) -> u8 {
        match self {
            DisplaySize::Display128x64 => 0,
        }
    }
}
