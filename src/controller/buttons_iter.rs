use crate::controller::pressed::Pressed;

/// An iterator over the buttons currently pressed in a given ButtonState. Iterates from the
/// highest bit to the lowest according to the SNES joypad register layout.
pub struct ButtonsIter {
    pub bitfield: u16,
    pub cursor_offset: u16,
}

impl ButtonsIter {
    const BIT_CURSOR: u16 = 0x8000;
}

impl Iterator for ButtonsIter {
    type Item = Pressed;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match Self::BIT_CURSOR.checked_shr(self.cursor_offset as u32) {
                None => break None,
                Some(bitmask) => {
                    self.cursor_offset = self.cursor_offset.saturating_add(1);
                    match (self.bitfield & bitmask) > 0 {
                        true => break Pressed::try_from_bit(bitmask),
                        false => continue,
                    }
                }
            }
        }
    }
}