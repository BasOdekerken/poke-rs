use crate::overworld::*;
use bevy::prelude::*;

pub fn tick_overworld_input_lock(
    time: Res<Time>,
    mut commands: Commands,
    lock: Option<ResMut<OverworldInputLock>>,
) {
    let Some(mut lock) = lock else {
        return;
    };

    lock.timer.tick(time.delta());
    if lock.timer.just_finished() {
        commands.remove_resource::<OverworldInputLock>();
    }
}

pub fn update_movement_state(
    time: Res<Time>,
    mut ev_step_finished: MessageWriter<StepFinished>,
    mut query: Query<(Entity, &mut MovementState, &mut Transform)>,
) {
    for (entity, mut state, mut transform) in &mut query {
        let MovementState::Moving { from, to, timer } = &mut *state else {
            continue;
        };
        
        timer.tick(time.delta());

        let t = timer.fraction();
        let pos = from.lerp(*to, t);

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;

        if timer.just_finished() {
            transform.translation.x = to.x;
            transform.translation.y = to.y;

            *state = MovementState::Idle;
            ev_step_finished.write(StepFinished { entity });
        }
    }
}
