use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

// create a new flyable camera with custom transform
pub fn fly_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0),
            ..default()
        }) //::default())
        .insert(FlyCamera::default());
}
