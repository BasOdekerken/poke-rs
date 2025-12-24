use bevy::prelude::*;
use super::{GridPos, MoveTween, TurnGrace, OverworldGridSettings};

pub fn snap_entity_to_grid(
    settings: Option<Res<OverworldGridSettings>>,
    mut commands: Commands,
    mut query: Query<(Entity, &GridPos, &mut Transform)>,
) {
    let Some(settings) = settings else {
        return;
    };

    for (entity, grid, mut transform) in &mut query {
        transform.translation.x = grid.x as f32 * settings.tile_size;
        transform.translation.y = grid.y as f32 * settings.tile_size;

        commands.entity(entity).remove::<MoveTween>();
        commands.entity(entity).remove::<TurnGrace>();
    }
}
