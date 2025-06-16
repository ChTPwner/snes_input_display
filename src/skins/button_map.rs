use crate::controller::pressed::Pressed;

use crate::skins::Button;

/// A wrapper over an array `[Button; 12]` indexable by `Pressed`. The array is internally ordered
/// by a button's bit ascending from lowest bit to highest.
#[derive(Debug)]
pub struct ButtonsMap(pub [Button; 12]);

impl std::ops::Index<Pressed> for ButtonsMap {
    type Output = Button;

    fn index(&self, pressed: Pressed) -> &Self::Output {
        let index = match pressed {
            Pressed::R => 0,
            Pressed::L => 1,
            Pressed::X => 2,
            Pressed::A => 3,
            Pressed::Right => 4,
            Pressed::Left => 5,
            Pressed::Down => 6,
            Pressed::Up => 7,
            Pressed::Start => 8,
            Pressed::Select => 9,
            Pressed::Y => 10,
            Pressed::B => 11,
        };

        &self.0[index]
    }
}

impl std::ops::IndexMut<Pressed> for ButtonsMap {
    fn index_mut(&mut self, pressed: Pressed) -> &mut Self::Output {
        let index = match pressed {
            Pressed::R => 0,
            Pressed::L => 1,
            Pressed::X => 2,
            Pressed::A => 3,
            Pressed::Right => 4,
            Pressed::Left => 5,
            Pressed::Down => 6,
            Pressed::Up => 7,
            Pressed::Start => 8,
            Pressed::Select => 9,
            Pressed::Y => 10,
            Pressed::B => 11,
        };

        &mut self.0[index]
    }
}
