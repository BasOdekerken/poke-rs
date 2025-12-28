use bevy::prelude::*;

#[derive(Component)]
pub struct OverworldCamera;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Resource)]
pub struct CameraFollowSettings {
    pub stiffness: f32,
}

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
pub enum MovementState {
    Idle,
    Moving {
        from: Vec2,
        to: Vec2,
        timer: Timer,
    },
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
