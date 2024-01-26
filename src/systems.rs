use crate::components::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 360.0;
// Player sprite size.
pub const CASTLE_SIZE: f32 = 64.0;
pub const NUMBER_OF_CASTLES: u32 = 4;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let player_asset_filename = "sprites/player.png";
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load(player_asset_filename),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_castles(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for index in 0..NUMBER_OF_CASTLES {
        let x = window.width() / (NUMBER_OF_CASTLES + 1) as f32 * (index + 1) as f32;
        let y = window.height() / 4.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/castle_2.png"),
                ..default()
            },
            Castle { hitpoints: 2 },
        ));
    }
}

pub fn spawn_bullet(mut comands: Commands,
                    keyboard_input: Res<Input<KeyCode>>,
                    mut player_query: Query<&Transform, With<Player>>,
) {
    // Wait untill the player presses space
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Get the player position, so we know where to spawn the bullet
        if let Ok(mut player) = player_query.get_single_mut() {
            println!("Space pressed and we are shooting!");
            comands.spawn(
                (Bullet {
                    position: Vec2::new(player.translation.x, player.translation.y),
                    speed: 2,
                }),
            );
        };
    }
}

pub fn move_bullet(mut commands: Commands, mut query: Query<(Entity, &mut Bullet)>) {
    for (entity, mut bullet) in query.iter_mut() {
        bullet.position.y += bullet.speed as f32;

        // Despawn if its outside of the screen
        if bullet.position.y > 800f32 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_sprite_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_sprite_size;
        let x_max = window.width() - half_sprite_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // TODO: make configurable
    const AMOUNT_OF_ROWS: u32 = 5;
    const ENEMY_SIZE: f32 = 64.0;
    let top_offset = window.height() / 4.0;

    println!("hi");
    for i in 0..AMOUNT_OF_ROWS {
        let level = AMOUNT_OF_ROWS - i;
        let size = ENEMY_SIZE / level as f32;
        println!("{i}, {level}, {size}");
        for j in 0..(window.width() / size) as usize {
            let new_j = (j as f32 * size);
            println!("{top_offset} {level} {i} {j} {size} {new_j}");
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(new_j, i as f32 * ENEMY_SIZE, 0.0),
                    texture: asset_server.load("sprites/player.png"),
                    ..default()
                },
                Enemy {
                    level,
                    is_dead: false,
                },
            ));
        }
    }
}
