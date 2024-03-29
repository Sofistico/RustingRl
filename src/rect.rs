use super::*;
use rltk::RandomNumberGenerator;
use specs::{World, WorldExt};

use crate::math_util;

pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub(crate) fn rng_pos(&self, mut rng: RandomNumberGenerator) -> (i32, i32) {
        (rng.range(self.x1, self.x2), rng.range(self.y1, self.y2))
    }

    pub(crate) fn rng_pos_index(&self, width: usize, mut rng: RandomNumberGenerator) -> i32 {
        let (x, y) = self.rng_pos(rng);
        math_util::to_index(x, y, width as i32)
    }
}
