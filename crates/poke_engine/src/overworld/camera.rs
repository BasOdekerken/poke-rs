use bevy::prelude::*;

use super::{OverworldCamera, CameraTarget, CameraFollowSettings};

pub fn camera_follow(
    time: Res<Time>,
    settings: Option<Res<CameraFollowSettings>>,
    mut query_camera: Query<&mut Transform, (With<OverworldCamera>, Without<CameraTarget>)>,
    query_target: Query<&mut Transform, (With<CameraTarget>, Without<OverworldCamera>)>,
) {
    let Ok(mut camera_transform) = query_camera.single_mut() else {
         return; 
    };
    let Ok(target_transform) = query_target.single() else {
         return; 
    };
    
    let target_pos = target_transform.translation;
    let camera_pos = camera_transform.translation;

    let desired = Vec3::new(target_pos.x, target_pos.y, camera_pos.z);

    let Some(settings) = settings else {
        camera_transform.translation = desired;
        return;
    };

    let k = settings.stiffness.max(0.0);
    let dt = time.delta_secs();
    let alpha = 1.0 - (-k * dt).exp();

    camera_transform.translation = camera_pos.lerp(desired, alpha);
}