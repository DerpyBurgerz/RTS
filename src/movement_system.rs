use bevy::prelude::*;
use bevy::prelude::MouseButton::Right;
use bevy::window::PrimaryWindow;
use crate::components::{Movable, Position};

#[derive(Component)]
struct Selected;

fn move_selected(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera2d, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut selected_units: Query<(Entity, &Position), (With<Movable>, With<Selected>)>
) {
    if !buttons.pressed(Right) {
        return;
    }

    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let (camera, elsething) = camera.into_inner();


}