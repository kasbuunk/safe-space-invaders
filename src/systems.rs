use crate::components::*;
use crate::events::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::MaterialMesh2dBundle};
use bevy::math::vec3;

use bevy_xpbd_2d::{math::*, prelude::*};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
// Player sprite size.
pub const CASTLE_SIZE: f32 = 64.0;
pub const NUMBER_OF_CASTLES: u32 = 4;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

const AMOUNT_OF_ROWS: u32 = 5;
const AMOUNT_OF_ENEMIES: u32 = 10;
const ENEMY_SIZE: f32 = 32.0;

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

pub fn start_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_music_filename = "audio/menu-music-loop.ogg";
    commands.spawn((
        AudioBundle {
            source: asset_server.load(menu_music_filename),
            settings: PlaybackSettings::LOOP,
            ..default()
        },
        MenuMusic {},
    ));
}

pub fn handle_game_start_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_music_query: Query<Entity, With<MenuMusic>>,
    mut game_over_music_query: Query<Entity, With<GameOverMusic>>,
    mut game_start_event_reader: EventReader<StartGame>,
) {
    match game_start_event_reader.read().next() {
        Some(_) => {
            // Stop other music.
            if let Ok(music) = menu_music_query.get_single_mut() {
                commands.entity(music).despawn();
            }
            if let Ok(music) = game_over_music_query.get_single_mut() {
                commands.entity(music).despawn();
            }

            // Start game over music.
            let menu_music_filename = "audio/game-start-music.ogg";
            commands.spawn((
                AudioBundle {
                    source: asset_server.load(menu_music_filename),
                    settings: PlaybackSettings::LOOP,
                    ..default()
                },
                GameStartMusic {},
            ));
        }
        None => (),
    }
}

pub fn handle_game_over_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_music_query: Query<Entity, With<MenuMusic>>,
    mut game_start_music_query: Query<Entity, With<GameStartMusic>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    match game_over_event_reader.read().next() {
        Some(_) => {
            // Stop other music.
            if let Ok(music) = menu_music_query.get_single_mut() {
                commands.entity(music).despawn();
            }
            if let Ok(music) = game_start_music_query.get_single_mut() {
                commands.entity(music).despawn();
            }

            // Start game over music.
            let menu_music_filename = "audio/game-over.ogg";
            commands.spawn((
                AudioBundle {
                    source: asset_server.load(menu_music_filename),
                    settings: PlaybackSettings::ONCE,
                    ..default()
                },
                GameOverMusic {},
            ));
        }
        None => (),
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
    mut castle_query: Query<(Entity, &mut Castle)>,
) {
    for (castle_entity, mut castle) in &mut castle_query {
        let castle_was_hit = false;

        if castle_was_hit {
            castle.hitpoints -= 1;
            if castle.hitpoints == 0 {
                // Despawn Castle with these hitpoints.
                commands.entity(castle_entity).despawn();
            }
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    // Wait until the player presses space
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Get the player position, so we know where to spawn the bullet
        if let Ok(player) = player_query.get_single() {
            commands.spawn(
                (SpriteBundle {
                    transform: Transform::from_xyz(player.translation.x, player.translation.y, 0.0).with_scale(vec3(10.0, 10.0, 10.0)),
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                }, Bullet {
                    speed: 500.0,
                },
                 Sensor,
                 RigidBody::Dynamic,
                 Collider::cuboid(100.0, 100.0)),
            );
        }

        let bullet_fire = "audio/schieten.ogg";
        commands.spawn(AudioBundle {
            source: asset_server.load(bullet_fire),
            settings: PlaybackSettings::ONCE,
            ..default()
        });
    }
}

pub fn move_bullet(
    mut commands: Commands,
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
    enemy_catalog: Res<EnemyCatalog>,
    mut start_game_event_reader: EventReader<StartGame>,
) {
    match start_game_event_reader.read().next() {
        Some(_) => {
            let window = window_query.get_single().unwrap();

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
                    let enemy = enemy_catalog.get_random_enemy();
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                new_j,
                                top_offset + i as f32 * ENEMY_SIZE,
                                0.0,
                            ),
                            texture: asset_server.load(format!("sprites/{enemy}.png")),
                            ..default()
                        },
                        Enemy {
                            level,
                            is_dead: false,
                        },
                        Sensor,
                        RigidBody::Dynamic,
                        Collider::cuboid(10.0, 10.0)
                    ));
                }
            }
        }
        None => {}
    }
}

pub fn enemy_movements(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    enemy_info: ResMut<EnemyInfo>,
) {
    const STEP: f32 = 0.5;
    for mut enemy in &mut enemies_query {
        match enemy_info.stage {
            EnemyStage::RIGHT => {
                enemy.translation.x += STEP;
            }
            EnemyStage::DOWN(_, _) => {
                enemy.translation.y -= STEP;
            }
            EnemyStage::LEFT => {
                enemy.translation.x -= STEP;
            }
        }
    }
}

pub fn update_enemy_info(
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    mut enemy_info: ResMut<EnemyInfo>,
) {
    if let EnemyStage::DOWN(down_amount, go_left) = enemy_info.stage {
        enemy_info.stage = if down_amount > 0 {
            EnemyStage::DOWN(down_amount - 1, go_left)
        } else if go_left {
            EnemyStage::LEFT
        } else {
            EnemyStage::RIGHT
        };

        return;
    }

    let window = window_query.get_single().unwrap();

    let mut max_x = f32::MIN;
    let mut min_x = f32::MAX;

    for enemy in &enemies_query {
        if enemy.translation.x > max_x {
            max_x = enemy.translation.x;
        }

        if enemy.translation.x < min_x {
            min_x = enemy.translation.x;
        }
    }

    const DOWN_AMOUNT: usize = 25; // 15px down

    let size = ENEMY_SIZE / 2.0;
    if min_x <= 0.5 + size {
        enemy_info.stage = EnemyStage::DOWN(DOWN_AMOUNT, false);
    } else if max_x >= window.width() - 0.5 - size {
        enemy_info.stage = EnemyStage::DOWN(DOWN_AMOUNT, true);
    }
}

pub fn enemy_hit_player(
    mut game_over_event_writer: EventWriter<GameOver>,
    mut lives: ResMut<Lives>,
    keyboard_input: Res<Input<KeyCode>>,
    score: Res<Score>,
) {
    if keyboard_input.pressed(KeyCode::L) {
        if lives.value <= 0 {
            game_over_event_writer.send(GameOver { score: score.value });
            return;
        }

        lives.value -= 1;
    }
}

pub fn setup_lives(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut start_game_event_reader: EventReader<StartGame>,
) {
    match start_game_event_reader.read().next() {
        Some(_) => {
            commands.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "Lives: ",
                        TextStyle {
                            font: asset_server.load("fonts/Sanspix-Regular.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                    ),
                    TextSection::from_style(TextStyle {
                        font: asset_server.load("fonts/Sanspix-Regular.ttf"),
                        font_size: 30.0,
                        ..default()
                    }),
                ])
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                }),
                LivesCounter,
            ));
        }
        None => (),
    }
}

pub fn update_lives(mut query: Query<&mut Text, With<LivesCounter>>, lives: Res<Lives>) {
    for mut text in &mut query {
        let value = lives.value;
        text.sections[1].value = format!("{value}");
    }
}
