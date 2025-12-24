use bevy::prelude::*;
use crate::overworld::*;

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

pub fn update_move_tween(
    time: Res<Time>,
    mut commands: Commands,
    mut ev_step_finished: MessageWriter<StepFinished>,
    mut query: Query<(Entity, &mut MoveTween, &mut Transform)>,
) {
    for (entity, mut tween, mut transform) in &mut query {
        tween.timer.tick(time.delta());

        let t = tween.timer.fraction();
        let pos = tween.from.lerp(tween.to, t);

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;

        if tween.timer.just_finished() {
            transform.translation.x = tween.to.x;
            transform.translation.y = tween.to.y;
            
            commands.entity(entity).remove::<MoveTween>();
            ev_step_finished.write(StepFinished { entity });
        }
    }
}