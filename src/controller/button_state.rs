use crate::controller::buttons_iter::ButtonsIter;

/// A `u16` backed bitfield representing a controller state according to the SNES joypad register
/// layout.
#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct ButtonState(u16);

impl ButtonState {
    /// Construct a `ButtonState` from little-endian bytes. The low and high bytes correspond to
    /// the low and high bytes of the SNES joypad registers.
    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        ButtonState(u16::from_le_bytes(bytes))
    }

    /// Provides an iterator over the buttons pressed in this `ButtonState` which returns
    /// `Option<Pressed>`.
    pub fn iter(&self) -> ButtonsIter {
        ButtonsIter {
            bitfield: self.0,
            cursor_offset: 0,
        }
    }
}
