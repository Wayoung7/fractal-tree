use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1920.,
        min_height: 1200.,
    };
    camera.transform.rotate_z(PI / 8.);
    clear_color.0 = Color::GRAY;
    commands.spawn(camera);
}
