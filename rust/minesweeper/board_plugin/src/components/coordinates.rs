use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16
}

// 2D point addition
impl Add for Coordinates{
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}

// 2D point substraction
impl Sub for Coordinates{
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(v.x),
            y: self.y.saturating_sub(v.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}