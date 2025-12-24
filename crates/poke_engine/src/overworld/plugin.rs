use bevy::prelude::*;

use super::{StepFinished, tick_overworld_input_lock, update_move_tween};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OverworldSet;
pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<StepFinished>()
            .add_systems(Update, (tick_overworld_input_lock, update_move_tween).in_set(OverworldSet));
    }
}