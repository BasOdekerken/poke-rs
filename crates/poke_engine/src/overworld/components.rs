use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Component)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct MoveTween {
    pub from: Vec2,
    pub to: Vec2,
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct TurnGrace {
    pub dir: Facing,
    pub timer: Timer,
}

#[derive(Resource)]
pub struct OverworldInputLock {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct OverworldGridSettings {
    pub tile_size: f32,
}

#[derive(Resource)]
pub struct OverworldMovementSettings {
    pub step_time: f32,
    pub turn_grace: f32,
}

#[derive(Resource)]
pub struct OverworldInputSettings {
    pub input_cooldown: f32,
}

#[derive(Message, Copy, Clone, Debug)]
pub struct StepFinished {
    pub entity: Entity,
}