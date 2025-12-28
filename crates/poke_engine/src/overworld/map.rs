use bevy::prelude::*;

#[derive(Resource)]
pub struct OverworldMap {
    pub width: i32,
    pub height: i32,
    blocked: Vec<bool>,
    encounter: Vec<bool>,
}

impl OverworldMap {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Map dimensions must be > 0");
        let size = (width * height) as usize;
        Self {
            width,
            height,
            blocked: vec![false; size],
            encounter: vec![false; size],
        }
    }

    #[inline]
    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.blocked[self.idx(x, y)]
    }

    pub fn is_encounter(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.encounter[self.idx(x, y)]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, blocked: bool, encounter: bool) {
        if !self.in_bounds(x, y) {
            return;
        }

        let idx = self.idx(x, y);
        self.blocked[idx] = blocked;
        self.encounter[idx] = encounter;
    }
}
