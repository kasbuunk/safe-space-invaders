use crate::components::*;
use crate::events::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
// Player sprite size.
pub const CASTLE_SIZE: f32 = 64.0;
pub const NUMBER_OF_CASTLES: u32 = 4;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub fn spawn_intro(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let intro_asset_filename = "images/intro.png";
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load(intro_asset_filename),
            ..default()
        },
        Intro {},
    ));
}

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let intro_asset_filename = "images/background.png";
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -10.0),
            texture: asset_server.load(intro_asset_filename),
            ..default()
        }),
    );
}


pub fn start_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut start_game_event_writer: EventWriter<StartGame>,
    mut intro_query: Query<(Entity, &Transform), With<Intro>>,
    mut game: ResMut<Game>,
) {
    if keyboard_input.pressed(KeyCode::Space) && !game.started {
        start_game_event_writer.send(StartGame {});
        if let Ok((intro_entity, intro_transform)) = intro_query.get_single_mut() {
            commands.entity(intro_entity).despawn();
        }
        game.started = true
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut start_game_event_reader: EventReader<StartGame>,
) {
    match start_game_event_reader.read().next() {
        Some(event) => {
            let player_asset_filename = "sprites/spaceship.png";
            let window: &Window = window_query.get_single().unwrap();

            let player_height = window.height() / 10.0;

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(window.width() / 2.0, player_height, 0.0),
                    texture: asset_server.load(player_asset_filename),
                    ..default()
                },
                Player {},
            ));
        }
        None => (),
    }
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
    mut start_game_event_reader: EventReader<StartGame>,
) {
    match start_game_event_reader.read().next() {
        Some(event) => {
            let window: &Window = window_query.get_single().unwrap();

            for index in 0..NUMBER_OF_CASTLES {
                let x = window.width() / (NUMBER_OF_CASTLES + 1) as f32 * (index + 1) as f32;
                let y = window.height() / 4.0;

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0),
                        texture: asset_server.load("sprites/castle.png"),
                        ..default()
                    },
                    Castle { hitpoints: 2 },
                ));
            }
        }
        None => (),
    }
}

pub fn bullet_hit_castle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    castle_query: Query<(Entity, &Transform), With<Castle>>,
    asset_server: Res<AssetServer>,
) {
    for (castle, transform) in castle_query.iter() {
        let castle_was_hit = false;

        if castle_was_hit {
            // Decrement hitpoints.
            // let new_hitpoints = castle.hitpoints;
            let new_hitpoints = 1;
            let position_x = 0.0;
            let position_y = 0.0;

            // Despawn Castle with these hitpoints.
            commands.entity(castle).despawn();
        }
    }
}

pub fn spawn_bullet(
    mut comands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    // Wait until the player presses space
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Get the player position, so we know where to spawn the bullet
        if let Ok(player) = player_query.get_single() {
            comands.spawn(
                (SpriteBundle {
                    transform: Transform::from_xyz(player.translation.x, player.translation.y, 0.0),
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                }, Bullet {
                    speed: 10,
                }),
            );
        }
    }
}

pub fn move_bullet(mut commands: Commands,
                   mut bullet_query: Query<(&mut Transform, Entity, &mut Bullet), With<Bullet>>,
                   time: Res<Time>,
) {
    for bullet in bullet_query.iter_mut() {
        let mut bullet_transform = bullet.0;
        let bullet_entity = bullet.1;
        let bullet_speed = bullet.2.speed;
        bullet_transform.translation.y += bullet_speed * time.delta_seconds();

        // Despawn if it's outside the screen
        if bullet_transform.translation.y > WINDOW_HEIGHT {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {

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
    mut start_game_event_reader: EventReader<StartGame>,
) {
    match start_game_event_reader.read().next() {
        Some(_) => {
            let window = window_query.get_single().unwrap();

            // TODO: make configurable
            const AMOUNT_OF_ROWS: u32 = 5;
            const AMOUNT_OF_ENEMIES: u32 = 10;
            const ENEMY_SIZE: f32 = 32.0;

            let top_offset = window.height() - ENEMY_SIZE * AMOUNT_OF_ROWS as f32;

            let window_padding = ENEMY_SIZE / 2.0;
            let window_width = window.width() - window_padding * 2.0;

            let padding_per_enemy =
                (window_width - (ENEMY_SIZE * AMOUNT_OF_ENEMIES as f32)) / AMOUNT_OF_ENEMIES as f32;

            for i in 0..AMOUNT_OF_ROWS {
                let level = AMOUNT_OF_ROWS - i;
                let size = ENEMY_SIZE as f32 + padding_per_enemy;
                for j in 0..(window.width() / size) as usize {
                    let new_j = j as f32 * size + window_padding + padding_per_enemy / 2.0;
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                new_j,
                                top_offset + i as f32 * ENEMY_SIZE,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/enemy-character.png"),
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
        None => {}
    }
}

pub fn enemy_hit_player(
    mut game_over_event_writer: EventWriter<GameOver>,
    mut lives: ResMut<Lives>,
    score: Res<Score>,
) {
    if lives.value <= 0 {
        game_over_event_writer.send(GameOver { score: score.value });
        return;
    }

    lives.value -= 1;
}
