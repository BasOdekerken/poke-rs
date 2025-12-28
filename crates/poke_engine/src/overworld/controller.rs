use bevy::prelude::*;

use super::{
    facing_delta, facing_pressed,
    input::{update_held_dirs, HeldDirs},
    Facing, GridPos, MoveTween, OverworldGridSettings, OverworldInputLock, OverworldMap,
    OverworldMovementSettings, TurnGrace,
};

#[derive(Component, Debug, Clone, Copy)]
pub struct OverworldPlayerController;

pub fn overworld_controller_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<OverworldMap>,
    lock: Option<Res<OverworldInputLock>>,
    grid: Res<OverworldGridSettings>,
    movement: Res<OverworldMovementSettings>,
    mut held: ResMut<HeldDirs>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut GridPos,
            &mut Facing,
            &Transform,
            Option<&mut TurnGrace>,
        ),
        (With<OverworldPlayerController>, Without<MoveTween>),
    >,
) {
    if lock.is_some() {
        return;
    }

    update_held_dirs(&keys, &mut held);

    let Some(dir) = held.active() else {
        return;
    };

    let Ok((entity, mut pos, mut facing, transform, turn_grace)) = query.single_mut() else {
        return;
    };

    // Turn grace handling prevent immediate move when tapping a new direction
    if let Some(mut turn_grace) = turn_grace {
        if !facing_pressed(&keys, turn_grace.dir) {
            commands.entity(entity).remove::<TurnGrace>();
            return;
        }

        turn_grace.timer.tick(time.delta());
        if !turn_grace.timer.just_finished() {
            return;
        }

        commands.entity(entity).remove::<TurnGrace>();
    }

    // Turn first
    if *facing != dir {
        *facing = dir;

        commands.entity(entity).insert(TurnGrace {
            dir,
            timer: Timer::from_seconds(movement.turn_grace, TimerMode::Once),
        });

        return;
    }

    // Then move
    let (dx, dy) = facing_delta(dir);
    let next_x = pos.x + dx;
    let next_y = pos.y + dy;

    if map.is_blocked(next_x, next_y) {
        return;
    }

    pos.x = next_x;
    pos.y = next_y;

    let from = Vec2::new(transform.translation.x, transform.translation.y);
    let to = Vec2::new(
        next_x as f32 * grid.tile_size,
        next_y as f32 * grid.tile_size,
    );

    commands.entity(entity).insert(MoveTween {
        from,
        to,
        timer: Timer::from_seconds(movement.step_time, TimerMode::Once),
    });
}
