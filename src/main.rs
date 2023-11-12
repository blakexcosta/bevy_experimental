mod camera_movement;
use bevy::render::view::window;
use camera_movement::*;

use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{Plugin, *};
use bevy::window::PrimaryWindow;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::{random, Rng};
use std::f32::consts::PI;
use std::thread::spawn;

// // TODO - Remove or refactor the use of these variables
// // variables for setting defaults for display size
// pub const HEIGHT: f32 = 1080.0;
// pub const WIDTH: f32 = 1920.0;

// // COMPONENTS
// // ----------------------------------------------
// // These are examples of components
// // new tower component with a Timer
// #[derive(Reflect, Component, Default)]
// #[reflect(Component)]
// pub struct Tower {
//     shooting_timer: Timer,
// }

// #[derive(Reflect, Component, Default)]
// #[reflect(Component)]
// pub struct Lifetime {
//     timer: Timer,
// }

// #[derive(Reflect, Component, Default)]
// #[reflect(Component)]
// pub struct Target {
//     speed: f32,
//     direction: Vec3,
// }

// #[derive(Reflect, Component, Default)]
// #[reflect(Component)]
// pub struct Health {
//     value: i32,
// }
// // ----------------------------------------------

// // CAN REMOVE
// // fn move_targets(
// //     mut commands: Commands,
// //     mut targets: Query<(Entity, &mut Target)>, // Entity is only one don't need & or &mut
// //     time: Res<Time>,
// // ) {
// //     // iterate over Lifetimes on bulet component
// //     // only gets entities with Lifetime component
// //     for (entity, mut target) in &mut targets {
// //         //target.transform.translation += target.direction * time.delta_seconds();

// //         // check if timer just finished and despawn bullet
// //         if lifetime.timer.just_finished() {
// //             commands.entity(entity).despawn_recursive(); // almost always use despawn_recursive
// //         }
// //     }
// // }
// // CAN REMOVE
// fn tower_shooting(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut towers: Query<&mut Tower>, // gets all tower components in game. can only use &mut or & for Query
//     time: Res<Time>,
// ) {
//     // get a random number between 0.0 and 4.0 for some fun
//     let mut rng = rand::thread_rng();
//     let random_float: f32 = rng.gen_range(0.0..4.0);
//     // iterate over tower components queried
//     for mut tower in &mut towers {
//         // tick the timer
//         tower.shooting_timer.tick(time.delta());
//         // check if timer just finished and spawn bullet
//         if tower.shooting_timer.just_finished() {
//             // create new transform for bullet
//             let spawn_transform = Transform::from_xyz(0.0, 0.7, random_float)
//                 .with_rotation(Quat::from_rotation_y(-PI / 2.0));
//             // pbr bundle - bullet
//             commands
//                 .spawn(PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
//                     material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
//                     transform: spawn_transform,
//                     ..default()
//                 })
//                 .insert(Lifetime {
//                     // add a lifetime component to the bullet
//                     timer: Timer::from_seconds(0.5, TimerMode::Once),
//                 })
//                 .insert(Name::new("Bullet"));
//         }
//     }
// }
// // CAN REMOVE
// fn bullet_despawn(
//     mut commands: Commands,
//     mut bullets: Query<(Entity, &mut Lifetime)>,
//     time: Res<Time>,
// ) {
//     // iterate over Lifetimes on bulet component
//     // only gets entities with Lifetime component
//     for (entity, mut lifetime) in &mut bullets {
//         lifetime.timer.tick(time.delta());
//         // check if timer just finished and despawn bullet
//         if lifetime.timer.just_finished() {
//             commands.entity(entity).despawn_recursive(); // almost always use despawn_recursive
//         }
//     }
// }
// // Simple camera spawn
// fn spawn_camera(mut commands: Commands) {
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }
// // Simple basic scene
// fn spawn_basic_scene(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // plane
//     commands
//         .spawn(PbrBundle {
//             mesh: meshes.add(shape::Plane::from_size(5.0).into()),
//             material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
//             ..default()
//         })
//         .insert(Name::new("Flat Plane")); //entity name

//     // capsule, Tower
//     commands
//         .spawn(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Capsule {
//                 radius: 0.5,
//                 ..Default::default()
//             })),
//             material: materials.add(Color::rgb(0.3, 0.2, 0.9).into()),
//             transform: Transform::from_xyz(0.0, 1.0, 0.0),
//             ..default()
//         })
//         .insert(Tower {
//             shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
//         }) // insert tower component
//         .insert(Name::new("Tower")); //entity name

//     // basic shape moving
//     commands
//         .spawn(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube {
//                 size: 0.1,
//                 ..Default::default()
//             })),
//             material: materials.add(Color::rgb(0.5, 0.7, 0.4).into()),
//             transform: Transform::from_xyz(0.0, 1.0, 0.0),
//             ..default()
//         })
//         .insert(Target {
//             speed: 4.0,
//             direction: Vec3::Y,
//         })
//         .insert(Name::new("Moving Target")); //entity name
// }
// // Simple light spawn
// fn spawn_light(mut commands: Commands) {
//     // light
//     commands
//         .spawn(PointLightBundle {
//             point_light: PointLight {
//                 intensity: 3750.0,
//                 shadows_enabled: true,
//                 ..default()
//             },
//             transform: Transform::from_xyz(4.0, 8.0, 4.0),
//             ..default()
//         })
//         .insert(Name::new("PointLight")); //entity name
// }

// // use bevy::prelude::*;
// // // use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
// // fn setup(mut commands: Commands) {
// //     commands
// //         .spawn(Camera3dBundle::default())
// //         .insert(FlyCamera::default());
// // }
pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player sprite size
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size
pub const DEBUG_AUDIO_TOGGLE: bool = false; // turns audio on or off for debugging purposes
pub const NUMBER_OF_STARS: i32 = 5;
pub const STAR_SIZE: f32 = 30.0; // This is star sprite pixels size
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    // have to implement if want to use as a resource
    fn default() -> Score {
        return Score { value: 0 };
    }
}

#[derive(Resource, Debug)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>, // player name and score
}
impl Default for HighScore {
    fn default() -> HighScore {
        return HighScore { scores: Vec::new() };
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    // have to implement if want to use as a resource
    fn default() -> StarSpawnTimer {
        return StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    // have to implement if want to use as a resource
    fn default() -> EnemySpawnTimer {
        return EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        };
    }
}

pub struct GameOver {
    pub score: u32,
}
// system for spawning a player
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

// system for spawning enemies
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            // create enemy with random direction, have to normalize in case vector is at an angle
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// system for spawning a camera
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // calculate pixel length of star
    let pixel_radius = STAR_SIZE / 2.0;

    for _ in 0..NUMBER_OF_STARS {
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        // keep star within bounds
        if random_x < pixel_radius {
            random_x += pixel_radius;
        } else if random_x > window.width() - pixel_radius {
            random_x -= pixel_radius;
        }
        if random_y < pixel_radius {
            random_y += pixel_radius;
        } else if random_y > window.height() - pixel_radius {
            random_y -= pixel_radius;
        }

        // || random_x > window.width() - pixel_radius
        // || random_y < pixel_radius
        // || random_y > window.height() - pixel_radius

        commands.spawn((
            // use a tuple to spawn multiple things
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            // create enemy with random direction, have to normalize in case vector is at an angle
            Star {},
        ));
    }
}

// system for player movement
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        // get player input
        if keyboard_input.pressed(KeyCode::W) {
            //info!("'W' currently pressed");
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            //info!("'A' currently pressed");
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            //info!("'S'  currently pressed");
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            //info!("'D'  currently pressed");
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        // normalize the vector
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // move player
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// system for keeping player object in bounds
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // if get_single_mut() returns Ok(), bind value and use it
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        // set the window bounds
        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;
        // get current player translation
        let mut translation = player_transform.translation;

        // bound the x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // bound the y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        // update position
        player_transform.translation = translation;
    }
}

// confine enemy movement to prevent from going out of bounds
pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // get window from window query
    let window = window_query.get_single().unwrap();

    // set minimum x and y bounds
    // set the window bounds
    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    // iterate over all enemies

    for mut transform in &mut enemy_query.iter_mut() {
        let mut translation = transform.translation;
        // check if current x position is outside of min and max bounds
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // check if current y position is outside of min and max bounds
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        // confine enemies to within the bounds of the window
        // enemy.direction.x = translation.x;
        // enemy.direction.y = translation.y;
        transform.translation = translation;
    }
}

// enemy movement
// notice the query uses a tuple to get multiple components
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    // iterate over enemies and move them
    for (mut transform, enemy) in &mut enemy_query {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0); // using a vector 3 just because it makes math easier
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // set the window bounds
    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    // get current player translation

    // iterate over each enemy, and reverse their vector if they hit the edge
    for (transform, mut enemy) in enemy_query.iter_mut() {
        // check if direction changes
        let mut direction_changed = false;

        // change direction if hit edge
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // play audio if edge was hit
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/impactTin_medium_000.ogg");
            let sound_effect_2 = asset_server.load("audio/impactTin_medium_002.ogg");
            // randomly play one of two sound effects
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            // quick audio toggle just to turn sounds on/off when debugging
            if DEBUG_AUDIO_TOGGLE {
                audio.play(sound_effect);
            }
        }
    }
}
fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2))) // set background color for scene
        // .add_startup_system(spawn_basic_scene) // create a basic scene
        // //.add_startup_system(spawn_camera) // spawn a camera
        // .add_startup_system(spawn_light) // spawn a light
        // .add_startup_system(fly_camera) // spawns a moveable camera for debugging
        .add_plugins(DefaultPlugins) // add default bevy plugins
        .init_resource::<Score>() // adds score resource to game with default value
        .init_resource::<StarSpawnTimer>() // adds a StarSpawnTimer Resource
        .init_resource::<EnemySpawnTimer>() // adds a EnemySpawnTimer Resource
        .init_resource::<HighScore>()
        .add_event::<GameOver>() // events are just normal structs
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_stars)
        // .register_type::<Tower>() // Register Custom Tower component
        // .register_type::<Lifetime>() // Register Lifetime component
        // .register_type::<Target>() // Register Target component
        // .register_type::<Health>() // Register Health component
        // .add_plugin(FlyCameraPlugin)
        //.add_plugin(WorldInspectorPlugin::new()) // add the plugin for setting up inspector
        //.add_plugin(FrameTimeDiagnosticsPlugin::default()) // add the plugin for displaying fps
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemy_movement)
        //.add_system(confine_enemy_movement) // bug in here
        .add_system(update_enemy_direction)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_enemies_over_time)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        //.add_system(tower_shooting)
        // .add_system(bullet_despawn) // despawn bullet
        // .add_system(keyboard_input_system)
        // .add_system(spawn_cube_on_click)
        .run();
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>, // used to emit game over event
    mut player_query: Query<(Entity, &Transform), With<Player>>, // Entity gets the generic entity specificied by the Player Filter (i.e. a player object)
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>, // used to get final score
) {
    // need a tuple because unpacking values that came FROM a tuple
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation); // get distance between player and enemy
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            // if distance less than sum of 2 radius, means entities are touching
            if distance < player_radius + enemy_radius {
                // Game over
                println!("Enemy hit player! Game Over!");
                // emit/send game over event
                game_over_event_writer.send(GameOver { score: score.value });
                // play audio
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                if DEBUG_AUDIO_TOGGLE {
                    audio.play(sound_effect);
                }
                //despawn entities (player)
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>, // Entity gets the generic entity specificied by the Player Filter (i.e. a player object)
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    // need a tuple because unpacking values that came FROM a tuple
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation); // get distance between player and enemy
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = ENEMY_SIZE / 2.0;
            // if distance less than sum of 2 radius, means entities are touching
            if distance < player_radius + star_radius {
                // Game over
                println!("Player hit player!");
                score.value += 1; // increment score
                let sound_effect = asset_server.load("audio/confirmation_001.ogg");
                if DEBUG_AUDIO_TOGGLE {
                    audio.play(sound_effect);
                }
                //despawn entities
                commands.entity(star_entity).despawn();
            }
        }
    }
}

// print out score if something changes
pub fn update_score(score: Res<Score>) {
    // is_changed() can be used on a resource if it has changed
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta()); // takes a duration rather than f32, ticks time down from last frame
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        // i.e. when timer hits zero (as timer counts down)
        let window = window_query.get_single().unwrap();
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        let pixel_radius = STAR_SIZE / 2.0;
        // keep star within bounds and from clipping outside window
        if random_x < pixel_radius {
            random_x += pixel_radius;
        } else if random_x > window.width() - pixel_radius {
            random_x -= pixel_radius;
        }
        if random_y < pixel_radius {
            random_y += pixel_radius;
        } else if random_y > window.height() - pixel_radius {
            random_y -= pixel_radius;
        }

        // spawn star
        commands.spawn((
            // use a tuple to spawn multiple things
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            // create enemy with random direction, have to normalize in case vector is at an angle
            Star {},
        ));
    }
}

// have to tick the timer, otherwise it won't work
pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta()); // takes a duration rather than f32, ticks time down from last frame
}

// spawn enemies over time. Make sure a system to tick the timer (i.e. tick_enemy_spawn_timer) is added!
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    // check if enemy spawn timer has finished
    if enemy_spawn_timer.timer.finished() {
        // i.e. when timer hits zero (as timer counts down)
        let window = window_query.get_single().unwrap();
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        let pixel_radius = ENEMY_SIZE / 2.0;
        // keep enemy within bounds and from clipping outside window
        if random_x < pixel_radius {
            random_x += pixel_radius;
        } else if random_x > window.width() - pixel_radius {
            random_x -= pixel_radius;
        }
        if random_y < pixel_radius {
            random_y += pixel_radius;
        } else if random_y > window.height() - pixel_radius {
            random_y -= pixel_radius;
        }

        // spawn enemy
        commands.spawn((
            // use a tuple to spawn multiple things
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            // create enemy with random direction, have to normalize in case vector is at an angle
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// system to exit the game
pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut exit_event_writer: EventWriter<AppExit>) {
    // exit game if escape is pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // std::process::exit(0);
        exit_event_writer.send(AppExit); // exit
    }
}

// temporary system to detect game over. Comment out system in main when not in use.
pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    // have to iterate over all events in a possible event reader, as multiple systems can send same event type
    for event in game_over_event_reader.iter() {
        //println!("Game Over!");
        println!("Final Score: {}", event.score.to_string());
    }
}

// system to update high scores
pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    // iterate over even reader
    for event in game_over_event_reader.iter() {
        // add the event score to the high scores
        high_scores
            .scores
            .push(("Player1".to_string(), event.score));
    }
}

// system just to check that high scores were updated
pub fn high_scores_updated(high_scores: Res<HighScore>) {
    // check if scores changed
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores.scores);
    }
}
// input system
// This system prints 'A' key state
// fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
//     if keyboard_input.pressed(KeyCode::W) {
//         info!("'W' currently pressed");
//     }

//     if keyboard_input.pressed(KeyCode::A) {
//         info!("'A' currently pressed");
//     }

//     if keyboard_input.pressed(KeyCode::S) {
//         info!("'S'  currently pressed");
//     }

//     if keyboard_input.pressed(KeyCode::D) {
//         info!("'D'  currently pressed");
//     }
// }

// fn spawn_cube_on_click(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mouse_button_input: Res<Input<MouseButton>>,
// ) {
//     let mut rng = rand::thread_rng();
//     let random_float: f32 = rng.gen_range(0.0..4.0);
//     let spawn_transform =
//         Transform::from_xyz(0.0, 0.7, random_float).with_rotation(Quat::from_rotation_y(-PI / 2.0));

//     // spawn new cube if mouse is pressed
//     if (mouse_button_input.just_pressed(MouseButton::Left)) {
//         commands
//             .spawn(PbrBundle {
//                 mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
//                 material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
//                 transform: spawn_transform,
//                 ..default()
//             })
//             .insert(Target {
//                 // add a lifetime component to the bullet
//                 speed: 4.0,
//                 direction: Vec3::Y,
//             })
//             .insert(Name::new("Circle"));
//     }
// }
