mod components;
mod unit_bundle;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use crate::components::Position;
use crate::unit_bundle::spawn_unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_unit,
            ))
        .run();
}
fn spawn_camera(mut commands: Commands) {commands.spawn(Camera2d);}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>){
    for (mut transform, position) in &mut positionables {
        // transform.translation = position.extend(0.);
    }
}
