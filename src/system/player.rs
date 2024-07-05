use crate::component::*;
use bevy::prelude::*;

pub fn input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    if keys.just_pressed(KeyCode::ArrowLeft) {
        try_move_player(&mut query, -1, 0);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        try_move_player(&mut query, 1, 0);
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        try_move_player(&mut query, 0, -1);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        try_move_player(&mut query, 0, 1);
    }
}

fn try_move_player(query: &mut Query<&mut Position, With<Player>>, dx: i32, dy: i32) {
    for mut position in query.iter_mut() {
        position.x += dx;
        position.y += dy;
    }
}
