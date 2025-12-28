use super::Facing;
use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct HeldDirs {
    dirs: Vec<Facing>,
}

impl HeldDirs {
    pub fn active(&self) -> Option<Facing> {
        self.dirs.first().copied()
    }

    fn remove(&mut self, dir: Facing) {
        self.dirs.retain(|&d| d != dir);
    }

    fn push_front(&mut self, dir: Facing) {
        self.remove(dir);
        self.dirs.insert(0, dir);
    }
}

#[inline]
pub fn update_held_dirs(keys: &ButtonInput<KeyCode>, held: &mut HeldDirs) {
    for dir in [Facing::North, Facing::South, Facing::East, Facing::West] {
        if just_down(keys, dir) {
            held.push_front(dir);
        }
    }

    for dir in [Facing::North, Facing::South, Facing::East, Facing::West] {
        if !is_down(keys, dir) {
            held.remove(dir);
        }
    }
}

#[inline]
fn is_down(keys: &ButtonInput<KeyCode>, dir: Facing) -> bool {
    match dir {
        Facing::North => keys.pressed(KeyCode::ArrowUp),
        Facing::South => keys.pressed(KeyCode::ArrowDown),
        Facing::East => keys.pressed(KeyCode::ArrowRight),
        Facing::West => keys.pressed(KeyCode::ArrowLeft),
    }
}

#[inline]
fn just_down(keys: &ButtonInput<KeyCode>, dir: Facing) -> bool {
    match dir {
        Facing::North => keys.just_pressed(KeyCode::ArrowUp),
        Facing::South => keys.just_pressed(KeyCode::ArrowDown),
        Facing::East => keys.just_pressed(KeyCode::ArrowRight),
        Facing::West => keys.just_pressed(KeyCode::ArrowLeft),
    }
}

pub fn facing_pressed(keys: &ButtonInput<KeyCode>, facing: Facing) -> bool {
    is_down(keys, facing)
}

pub fn facing_delta(dir: Facing) -> (i32, i32) {
    match dir {
        Facing::North => (0, 1),
        Facing::South => (0, -1),
        Facing::East => (1, 0),
        Facing::West => (-1, 0),
    }
}
