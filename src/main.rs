mod camera;
mod fractal_tree;
mod sketch;

use bevy::prelude::*;
use camera::CameraPlugin;
use fractal_tree::FractalTreePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FractalTreePlugin)
        .add_plugins(CameraPlugin)
        .run();
}
