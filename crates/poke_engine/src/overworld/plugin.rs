use bevy::prelude::*;

use super::{
    controller::overworld_controller_system, input::HeldDirs, tick_overworld_input_lock,
    update_move_tween, StepFinished,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct OverworldSet;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeldDirs>()
            .add_message::<StepFinished>()
            .add_systems(
                Update,
                (
                    tick_overworld_input_lock,
                    update_move_tween,
                    overworld_controller_system,
                )
                    .in_set(OverworldSet),
            );
    }
}
