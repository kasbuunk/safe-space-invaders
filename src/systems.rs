use crate::components::*;
use crate::events::*;
use crate::resources::*;
use std::string::ToString;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::audio::{PlaybackMode, Volume};
use bevy::math::vec3;
use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::MaterialMesh2dBundle};

use bevy_xpbd_2d::{math::*, prelude::*};
use rand::{random, Rng};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
// Player sprite size.
pub const CASTLE_HEIGHT: f32 = 70.0;
pub const CASTLE_WIDTH: f32 = 30.0;
pub const NUMBER_OF_CASTLES: u32 = 4;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

const AMOUNT_OF_ROWS: u32 = 5;
const AMOUNT_OF_ENEMIES: u32 = 10;
const ENEMY_SIZE: f32 = 32.0;

const GET_HIT_SOUNDS: [&str; 8] = [
    "audio/daanhit.ogg",
    "audio/erhanhit.ogg",
    "audio/Frankhit.ogg",
    "audio/hushit.ogg",
    "audio/jeroenhit.ogg",
    "audio/kashit.ogg",
    "audio/Ryanhit.ogg",
    "audio/stormhit.ogg",
];

pub fn spawn_game_intro(
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
        IntroScreen {},
    ));
}

pub fn spawn_game_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_over_screen_query: Query<Entity, With<GameOverScreen>>,
    asset_server: Res<AssetServer>,
    mut game_start_event_reader: EventReader<GameStartRequested>,
) {
    match game_start_event_reader.read().next() {
        Some(_) => {
            // Remove game over screen.
            if let Ok(screen) = game_over_screen_query.get_single_mut() {
                commands.entity(screen).despawn();
            }

            // Spawn new screen.
            let intro_asset_filename = "images/background.png";
            let window: &Window = window_query.get_single().unwrap();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        window.width() / 2.0,
                        window.height() / 2.0,
                        -10.0,
                    ),
                    texture: asset_server.load(intro_asset_filename),
                    ..default()
                },
                GameScreen {},
            ));
        }
        None => (),
    }
}

pub fn start_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut start_game_event_writer: EventWriter<GameStartRequested>,
    mut intro_query: Query<(Entity, &Transform), With<IntroScreen>>,
    mut game: ResMut<Game>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) && (*game == Game::INTRO || *game == Game::ENDED) {
    {
        start_game_event_writer.send(GameStartRequested {});
        if let Ok((intro_entity, intro_transform)) = intro_query.get_single_mut() {
            commands.entity(intro_entity).despawn();
        }
        *game = Game::LOADING;
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
    mut score_text_query: Query<Entity, With<ScoreText>>,
    mut game_start_event_reader: EventReader<GameStartRequested>,
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
            if let Ok(text) = score_text_query.get_single_mut() {
                commands.entity(text).despawn();
            }

            // Start game over music.
            let game_start_effect_filename = "audio/game-start-music.ogg";
            commands.spawn((
                AudioBundle {
                    source: asset_server.load(game_start_effect_filename),
                    settings: PlaybackSettings::LOOP,
                    ..default()
                },
                GameStartMusic {},
            ));
            let game_start_filename = "audio/game-start-music.ogg";
            commands.spawn((
                AudioBundle {
                    source: asset_server.load(game_start_filename),
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
            for music in menu_music_query.iter_mut() {
                commands.entity(music).despawn();
            }
            for music in game_start_music_query.iter_mut() {
                commands.entity(music).despawn();
            }

            // Start game over music.
            let menu_music_filename = "audio/game-over.ogg";
            commands.spawn((
                AudioBundle {
                    source: asset_server.load(menu_music_filename),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                },
                GameOverMusic {},
            ));
        }
        None => (),
    }
}

pub fn spawn_player(
    mut loading_flags: ResMut<LoadingFlags>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut start_game_event_reader: EventReader<GameStartRequested>,
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
                Sensor,
                RigidBody::Dynamic,
                Collider::cuboid(50.0, 50.0),
            ));

            loading_flags.player = true;
        }
        None => (),
    }
}

pub fn reset_lives(
    mut lives: ResMut<Lives>,
    mut start_game_event_reader: EventReader<GameStartRequested>,
) {
    match start_game_event_reader.read().next() {
        Some(event) => {
            lives.value = NUMBER_OF_LIVES;
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
    mut loading_flags: ResMut<LoadingFlags>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut start_game_event_reader: EventReader<GameStartRequested>,
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
                    Sensor,
                    RigidBody::Dynamic,
                    Collider::cuboid(CASTLE_WIDTH, CASTLE_HEIGHT),
                ));
            }

            loading_flags.castles = true;
        }
        None => (),
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
) {
    // Wait until the player presses space
    if keyboard_input.just_pressed(KeyCode::Space) && *game == Game::STARTED {
        // Get the player position, so we know where to spawn the bullet
        if let Ok(player) = player_query.get_single() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(player.translation.x, player.translation.y, 0.0),
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                },
                Bullet { speed: 500.0 },
                Sensor,
                RigidBody::Dynamic,
                Collider::cuboid(15.0, 10.0),
            ));
        }

        let bullet_fire = "audio/schieten.ogg";
        commands.spawn(AudioBundle {
            source: asset_server.load(bullet_fire),
            settings: PlaybackSettings::DESPAWN,
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

pub fn move_enemy_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, Entity, &mut EnemyBullet), With<EnemyBullet>>,
    time: Res<Time>,
) {
    for bullet in bullet_query.iter_mut() {
        let mut bullet_transform = bullet.0;
        let bullet_entity = bullet.1;
        let bullet_speed = bullet.2.speed;
        bullet_transform.translation.y -= bullet_speed * time.delta_seconds();

        // Despawn if it's outside the screen
        if bullet_transform.translation.y < 0.5 {
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
    mut loading_flags: ResMut<LoadingFlags>,
    mut start_game_event_reader: EventReader<GameStartRequested>,
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
                        Collider::cuboid(10.0, 10.0),
                    ));
                }
            }

            loading_flags.enemies = true;
        }
        None => {}
    }
}

pub fn enemy_movements(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    enemy_info: ResMut<EnemyInfo>,
    time: Res<Time>,
) {
    const STEP: f32 = 50.0;
    for mut enemy in &mut enemies_query {
        match enemy_info.stage {
            EnemyStage::RIGHT => {
                enemy.translation.x += STEP * time.delta_seconds();
            }
            EnemyStage::DOWN(_, _) => {
                enemy.translation.y -= STEP * time.delta_seconds();
            }
            EnemyStage::LEFT => {
                enemy.translation.x -= STEP * time.delta_seconds();
            }
        }
    }
}

pub fn enemy_shoot(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for enemy in &enemies_query {
        // bullet shoot chance based on height of enemy. between 0 and 1
        // let shoot_chance = 1.0 - (enemy.translation.y / window.height());
        let rnd = rand::random::<f32>();
        if rnd > (0.9998 - (1.0 - (enemy.translation.y / window.height())) / 1000.0) {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(enemy.translation.x, enemy.translation.y, 0.0),
                    texture: asset_server.load("sprites/bitterbal.png"),
                    ..default()
                },
                EnemyBullet { speed: 200.0 },
                Sensor,
                RigidBody::Dynamic,
                Collider::ball(21.0),
            ));

            let bullet_fire = "audio/schieten.ogg";
            commands.spawn(AudioBundle {
                source: asset_server.load(bullet_fire),
                settings: PlaybackSettings::ONCE,
                ..default()
            });
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
    let mut min_y = f32::MAX;

    for enemy in &enemies_query {
        if enemy.translation.x > max_x {
            max_x = enemy.translation.x;
        }

        if enemy.translation.x < min_x {
            min_x = enemy.translation.x;
        }

        if enemy.translation.y < min_y {
            min_y = enemy.translation.y;
        }
    }

    const DOWN_AMOUNT: usize = 25; // 15px down
    const CASTLE_HEIGHT: f32 = 70.0;

    let wall_border = (window.height() / 4.0) + (ENEMY_SIZE / 2.0) + CASTLE_HEIGHT;
    let stand_still = min_y <= wall_border;

    let size = ENEMY_SIZE / 2.0;
    if min_x <= 0.5 + size {
        enemy_info.stage = if stand_still {
            EnemyStage::RIGHT
        } else {
            EnemyStage::DOWN(DOWN_AMOUNT, false)
        };
    } else if max_x >= window.width() - 0.5 - size {
        enemy_info.stage = if stand_still {
            EnemyStage::LEFT
        } else {
            EnemyStage::DOWN(DOWN_AMOUNT, true)
        };
    }
}

pub fn setup_lives(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut start_game_event_reader: EventReader<GameStartRequested>,
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

pub fn bullet_hits_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut collision_query: Query<((Entity, &mut Bullet), &CollidingEntities)>,
    enemy_query: Query<&Enemy>,
) {
    for ((entity, mut bullet), colliding_entities) in collision_query.iter_mut() {
        for colliding_entity in colliding_entities.iter() {
            if enemy_query.get(*colliding_entity).is_ok() {
                commands.entity(*colliding_entity).despawn();
                commands.entity(entity).despawn();
                call_random_hit_sound(commands, asset_server);
                return;
            }
        }
    }
}

pub fn game_loaded(
    mut loading_flags: ResMut<LoadingFlags>,
    mut start_game_event_writer: EventWriter<GameStartRequested>,
    mut game: ResMut<Game>,
) {
    if *game != Game::LOADING {
        return;
    }

    if loading_flags.enemies && loading_flags.castles && loading_flags.player {
        *game = Game::STARTED;
        loading_flags.enemies = false;
        loading_flags.player = false;
        loading_flags.castles = false;
    }
}

pub fn detect_game_won(
    enemy_query: Query<&Enemy>,
    score: Res<Score>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut game: ResMut<Game>,
) {
    if *game != Game::STARTED {
        return;
    }

    if enemy_query.is_empty() {
        *game = Game::ENDED;
        game_over_event_writer.send(GameOver {
            won: true,
            score: score.value,
        });
    }
}

pub fn enemy_bullet_hits_player(
    mut commands: Commands,
    mut collision_query: Query<((Entity, &mut EnemyBullet), &mut CollidingEntities)>,
    mut player_query: Query<(Entity, &mut Player)>,
    mut lives: ResMut<Lives>,
    score: Res<Score>,
    mut game_over_event_writer: EventWriter<GameOver>,
) {
    for ((bullet_entity, mut bullet), mut colliding_entities) in collision_query.iter_mut() {
        for player_entity in colliding_entities.iter() {
            if let Ok((ent, mut cast)) = player_query.get_mut(*player_entity) {
                commands.entity(bullet_entity).despawn();
                if lives.value > 0 {
                    lives.value -= 1;
                }
                if lives.value == 0 {
                    game_over_event_writer.send(GameOver {
                        won: false,
                        score: score.value,
                    });
                }
                return;
            }
        }
    }
}

pub fn enemy_bullet_hits_castle(
    mut commands: Commands,
    mut collision_query: Query<((Entity, &mut EnemyBullet), &mut CollidingEntities)>,
    mut castle_query: Query<(Entity, &mut Castle)>,
) {
    for ((bullet_entity, mut bullet), mut colliding_entities) in collision_query.iter_mut() {
        for castle_entity in colliding_entities.iter() {
            if let Ok((ent, mut cast)) = castle_query.get_mut(*castle_entity) {
                cast.hitpoints -= 1;
                if cast.hitpoints == 0 {
                    // Despawn Castle with 0 hitpoints.
                    commands.entity(ent).despawn();
                }
                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }
}

pub fn bullet_hits_castle(
    mut commands: Commands,
    mut collision_query: Query<((Entity, &mut Bullet), &CollidingEntities)>,
    castle_query: Query<&Castle>,
) {
    for ((bullet_entity, mut bullet), colliding_entities) in collision_query.iter_mut() {
        for castle_entity in colliding_entities.iter() {
            if castle_query.get(*castle_entity).is_ok() {
                // Don't mutate the Castle.
                // commands.entity(*colliding_entity).despawn();
                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }
}

pub fn call_random_hit_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng(); // Create a random number generator

    // Generate a random number between 0 and the length of the array (exclusive)
    let random_index = rng.gen_range(0..GET_HIT_SOUNDS.len());

    let get_hit_sound = GET_HIT_SOUNDS[random_index];
    commands.spawn(AudioBundle {
        source: asset_server.load(get_hit_sound),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

pub fn handle_game_over(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut game_over_event_reader: EventReader<GameOver>,
    mut game: ResMut<Game>,
    player_query: Query<Entity, With<Player>>,
    castle_query: Query<Entity, With<Castle>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut high_score: ResMut<HighScore>,
    score: Res<Score>,
) {
    match game_over_event_reader.read().next() {
        Some(event) => {
            *game = Game::ENDED;

            let mut screen_asset_filename = "images/game-won.png";
            let window: &Window = window_query.get_single().unwrap();

            if event.won {
                if score.value > high_score.value {
                    high_score.value = score.value;
                }
            } else {
                screen_asset_filename = "images/game-lost.png";
            }

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        window.width() / 2.0,
                        window.height() / 2.0,
                        0.0,
                    )
                    .with_scale(Vec3::splat(0.25)),
                    texture: asset_server.load(screen_asset_filename),
                    ..default()
                },
                GameOverScreen {},
            ));

            for player in player_query.iter() {
                commands.entity(player).despawn();
            }

            for castle in castle_query.iter() {
                commands.entity(castle).despawn();
            }

            for enemy in enemy_query.iter() {
                commands.entity(enemy).despawn();
            }

            commands.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        format!("Score: {0}", score.value),
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
                    top: Val::Px(100.0),
                    left: Val::Px(100.0),
                    ..default()
                }),
                ScoreText,
            ));

            // [todo] on key press (space):
            // despawn game over screens
            // set `game.started = false;` -> will trigger new game
        }
        None => (),
    };
}
