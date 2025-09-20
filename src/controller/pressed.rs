#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u16)]
pub enum Pressed {
    R = 0x0010,
    L = 0x0020,
    X = 0x0040,
    A = 0x0080,
    Right = 0x0100,
    Left = 0x0200,
    Down = 0x0400,
    Up = 0x0800,
    Start = 0x1000,
    Select = 0x2000,
    Y = 0x4000,
    B = 0x8000,
}

impl Pressed {
    /// Accepts a `u16` with a single bit set according to the SNES joypad register layout and
    /// returns `Option<Pressed>` where None represents no buttons pushed. Caller is responsible
    /// for ensuring that the value passed in is zero or a single, valid bit. Otherwise the
    /// function will panic.
    pub fn try_from_bit(bit: u16) -> Option<Self> {
        debug_assert!(bit.is_power_of_two() || bit == 0);
        match bit {
            0x0000 => None,
            0x0010 => Some(Pressed::R),
            0x0020 => Some(Pressed::L),
            0x0040 => Some(Pressed::X),
            0x0080 => Some(Pressed::A),
            0x0100 => Some(Pressed::Right),
            0x0200 => Some(Pressed::Left),
            0x0400 => Some(Pressed::Down),
            0x0800 => Some(Pressed::Up),
            0x1000 => Some(Pressed::Start),
            0x2000 => Some(Pressed::Select),
            0x4000 => Some(Pressed::Y),
            0x8000 => Some(Pressed::B),
            _ => None,
        }
    }
}
