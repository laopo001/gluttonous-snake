use super::components::Bean;
use crate::snake::components::Snake;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

pub fn spawn_bean(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_query: Query<&Transform, With<Snake>>,
    bean_query: Query<&Bean>,
    asset_server: Res<AssetServer>,
) {
    // no need to spawn if bean still exists
    // TODO: why bean_query is still not empty when bean has been de-spawned?
    if !bean_query.is_empty() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    // TODO: bean should be spawned at a random position where the snake not collides
    if let Ok(snake_transform) = snake_query.get_single() {
        println!("{}", snake_transform.translation);
    }
    commands.spawn((
        SpriteBundle {
            // TODO: make sure that snake and bean be in the same grid
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("bean.png"),
            ..default()
        },
        Bean {},
    ));
}