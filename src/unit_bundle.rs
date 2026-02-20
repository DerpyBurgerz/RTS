use bevy::prelude::*;
use crate::components::{Position};

#[derive(Bundle)]
struct UnitBundle {
    pub position: Position,
}

pub fn spawn_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(10.));
    let material = materials.add(Color::BLACK);
    commands.spawn((
        UnitBundle {
            position: Position(Vec2::new(0., 0.))
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        ));
}