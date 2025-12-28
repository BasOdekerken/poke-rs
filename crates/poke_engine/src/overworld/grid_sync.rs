use bevy::prelude::*;
use super::{GridPos, MovementState, TurnGrace, OverworldGridSettings};

pub fn sync_transform_to_grid(
    settings: Option<Res<OverworldGridSettings>>,
    mut commands: Commands,
    mut query: Query<(Entity, &GridPos, &mut Transform, Option<&mut MovementState>)>,
) {
    let Some(settings) = settings else {
        return;
    };

    for (entity, grid, mut transform, move_state) in &mut query {
        transform.translation.x = grid.x as f32 * settings.tile_size;
        transform.translation.y = grid.y as f32 * settings.tile_size;

        if let Some(mut state) = move_state {
            *state = MovementState::Idle;
        }

        commands.entity(entity).remove::<TurnGrace>();
    }
}
