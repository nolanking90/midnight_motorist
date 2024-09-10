use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use rand::prelude::*;
use std::time::Duration;

const WINSCALE: f32 = 1.5;
const WINWIDTH: f32 = 1920.0 / WINSCALE;
const WINHEIGHT: f32 = 1080.0 / WINSCALE;
const CARHEIGHT: f32 = 105.0 / WINSCALE;
const CARWIDTH: f32 = 135.0 / WINSCALE;
const YSPEED: f32 = 1000.0;
const OBSTACLE_WIDTH: f32 = 135.0 / WINSCALE;
const OBSTACLE_HEIGHT: f32 = 106.0 / WINSCALE;
const Y_VALUES: [f32; 4] = [
    (OBSTACLE_HEIGHT / 2.0) + 20.0,
    135.0 + (OBSTACLE_HEIGHT / 2.0) + 10.0,
    2.0 * 135.0 + (OBSTACLE_HEIGHT / 2.0) + 15.0,
    3.0 * 135.0 + (OBSTACLE_HEIGHT / 2.0) - 10.0,
];

#[derive(Component)]
struct CameraMarker;

// TODO: SCORE, LAP, SPEED
fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), CameraMarker));
    commands.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(50.0 / 1080.0 * 100.0),
            left: Val::Percent(10.0 / 1920.0 * 100.0),
            width: Val::Percent(5.0),
            height: Val::Percent(5.0),
            ..Default::default()
        },
        image: UiImage {
            texture: asset_server.load("1079.png"),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(115.0 / 1080.0 * 100.0),
            left: Val::Percent(10.0 / 1920.0 * 100.0),
            width: Val::Percent(6.0),
            height: Val::Percent(5.0),
            ..Default::default()
        },
        image: UiImage {
            texture: asset_server.load("1104.png"),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("1058.png"), // Background
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: WINWIDTH * 3.0,
                    y: WINHEIGHT,
                }),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.0 / WINSCALE,
        },
        Background,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("1058.png"), // Background
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: WINWIDTH,
                    y: WINHEIGHT,
                }),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_xyz(WINWIDTH * -1.0, 0.0, 0.0),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: false,
            tile_y: false,
            stretch_value: 1.0 / WINSCALE,
        },
        Background,
    ));
}

#[derive(Component)]
struct Background;

fn update_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Query<&Transform, (With<CameraMarker>, Without<Background>)>,
    backgrounds: Query<(Entity, &Transform), (With<Background>, Without<CameraMarker>)>,
) {
    for (background_entity, background_transform) in backgrounds.iter() {
        if background_transform.translation.x < camera.single().translation.x - WINWIDTH * 4.0 {
            commands.entity(background_entity).despawn();
        }
    }
    let road_remaining = backgrounds.iter().any(|(_, background_transform)| {
        background_transform.translation.x > camera.single().translation.x
    });
    if !road_remaining {
        let road_length_factor = (camera.single().translation.x / (WINWIDTH * 3.0)).floor() + 1.0;

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("1058.png"), // Background
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: WINWIDTH * 3.0,
                        y: WINHEIGHT,
                    }),
                    anchor: bevy::sprite::Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_xyz(road_length_factor * 3.0 * WINWIDTH, 0.0, 0.0),
                ..default()
            },
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0 / WINSCALE,
            },
            Background,
        ));
    }
}

#[derive(Component)]
struct Car {
    speed: Vec2,
    state: CarState,
    score: f32,
    sprite_index: usize,
    texture_list: Vec<Handle<Image>>,
    frame_timer: Timer,
    position: Vec3,
}

#[derive(PartialEq)]
pub enum CarState {
    Moving,
    Crashed,
}

fn spawn_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut texture_list: Vec<Handle<Image>> = Vec::new();
    for n in 0..20 {
        texture_list.push(asset_server.load((1084 + n).to_string() + ".png"));
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: CARWIDTH,
                    y: CARHEIGHT,
                }),
                ..default()
            },
            texture: texture_list[0].clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Car {
            speed: Vec2 { x: 0.0, y: YSPEED },
            score: 0.0,
            state: CarState::Moving,
            frame_timer: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Repeating),
            sprite_index: 0,
            texture_list,
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ));
}

fn update_car(
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cars: Query<(&mut Car, &mut Transform, &mut Handle<Image>), Without<CameraMarker>>,
    camera: Query<&Transform, With<CameraMarker>>,
) {
    for (mut car, mut transform, mut sprite) in cars.iter_mut() {
        match car.state {
            CarState::Moving => {
                if button_input.pressed(KeyCode::KeyW) || button_input.pressed(KeyCode::ArrowUp) {
                    car.position.y += car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -WINHEIGHT / 2.0 + CARHEIGHT / 2.0,
                        WINHEIGHT / 2.0 - CARHEIGHT / 2.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyS) || button_input.pressed(KeyCode::ArrowDown) {
                    car.position.y -= car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -WINHEIGHT / 2.0 + CARHEIGHT / 2.0,
                        WINHEIGHT / 2.0 - CARHEIGHT / 2.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyD) || button_input.pressed(KeyCode::ArrowRight)
                {
                    car.position.x += car.speed.y * time.delta_seconds();
                    car.position.x = car.position.x.clamp(
                        camera.single().translation.x - WINWIDTH / 4.0,
                        camera.single().translation.x + WINWIDTH / 4.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyA) || button_input.pressed(KeyCode::ArrowLeft) {
                    car.position.x -= car.speed.y * time.delta_seconds();
                    car.position.x = car.position.x.clamp(
                        camera.single().translation.x - WINWIDTH / 4.0,
                        camera.single().translation.x + WINWIDTH / 4.0,
                    );
                }

                car.position.x += car.speed.x * time.delta_seconds();
                transform.translation = car.position;
                if car.speed.x < 1100.00 {
                    car.speed.x += 75.0 * time.delta_seconds();
                }
                if car.speed.x > 500.0 {
                    car.score += 10.0 * time.delta_seconds();
                }
            }
            CarState::Crashed => {
                car.speed.x = 100.0;
                car.position.x += car.speed.x * time.delta_seconds();
                transform.translation = car.position;

                if car.frame_timer.paused() {
                    car.frame_timer.unpause();
                }
                car.frame_timer.tick(time.delta());

                if car.sprite_index == 19 {
                    car.sprite_index = 0;
                } else {
                    car.sprite_index += 1;
                    *sprite = car.texture_list[car.sprite_index].clone();
                }

                if car.frame_timer.just_finished() {
                    car.sprite_index = 0;
                    car.state = CarState::Moving;
                    *sprite = car.texture_list[0].clone();
                }
            }
        }
    }
}

fn camera_tracking(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<CameraMarker>>,
    player: Query<&Car>,
) {
    camera.single_mut().translation.x += player.single().speed.x * time.delta_seconds();
}

#[derive(Component)]
struct Score {
    digits: Vec<Handle<Image>>,
}

#[derive(Component)]
struct ScoreDigit;

#[derive(Component)]
struct SpeedDigit;

fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut digits: Vec<Handle<Image>> = Vec::new();
    for n in 0..10 {
        digits.push(asset_server.load((1687 + n).to_string() + ".png"));
    }

    commands.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(50.0 / 1080.0 * 100.0),
            left: Val::Percent(76.0),
            width: Val::Percent(12.0),
            height: Val::Percent(5.0),
            ..Default::default()
        },
        image: UiImage {
            texture: asset_server.load("2118.png"),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn(Score { digits });
}

fn update_laps(
    mut commands: Commands,
    camera: Query<&Transform, With<CameraMarker>>,
    score: Query<&Score>,
) {
    let digit = camera.single().translation.x / (WINWIDTH * WINSCALE) / 10.0;

    commands.spawn(ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(50.0 / 1080.0 * 100.0),
            left: Val::Percent(10.0 / 1920.0 * 100.0 + 5.0),
            width: Val::Percent(3.0),
            height: Val::Percent(5.0),
            ..Default::default()
        },
        image: UiImage {
            texture: score.single().digits[digit as usize].clone(),
            ..default()
        },
        ..Default::default()
    });
}

fn update_score(
    mut commands: Commands,
    cars: Query<&Car>,
    score: Query<&Score>,
    prev_score_digits: Query<Entity, With<ScoreDigit>>,
    prev_speed_digits: Query<Entity, With<SpeedDigit>>,
) {
    for prev_digit in prev_score_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let player_score = cars.single().score.floor() as u32;
    let score_string = player_score.to_string();
    let temp = score_string.chars();
    let num_digits = temp.clone().count();
    let mut left_pos = WINWIDTH - 10.0 - (0.03 * WINWIDTH * num_digits as f32);

    for char in temp {
        let digit = char.to_digit(10).unwrap_or_default();
        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::horizontal(Val::Auto),
                    top: Val::Percent(50.0 / 1080.0 * 100.0),
                    left: Val::Px(left_pos),
                    width: Val::Percent(3.0),
                    height: Val::Percent(5.0),
                    ..Default::default()
                },
                image: UiImage {
                    texture: score.single().digits[digit as usize].clone(),
                    ..default()
                },
                ..Default::default()
            },
            ScoreDigit,
        ));
        left_pos += 0.03 * WINWIDTH;
    }

    for prev_digit in prev_speed_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let player_speed = (cars.single().speed.x / 10.0).floor() as u32;
    let speed_string = player_speed.to_string();
    let temp = speed_string.chars();
    let mut left_pos = 20.0 + 0.06 * WINWIDTH;

    for char in temp {
        let digit = char.to_digit(10).unwrap_or_default();
        commands.spawn((
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::horizontal(Val::Auto),
                    top: Val::Percent(115.0 / 1080.0 * 100.0),
                    left: Val::Px(left_pos),
                    width: Val::Percent(3.0),
                    height: Val::Percent(5.0),
                    ..Default::default()
                },
                image: UiImage {
                    texture: score.single().digits[digit as usize].clone(),
                    ..default()
                },
                ..Default::default()
            },
            SpeedDigit,
        ));
        left_pos += 0.03 * WINWIDTH;
    }
}

#[derive(Component)]
struct Obstacle {
    speed: f32,
}

fn update_obstacles(
    mut commands: Commands,
    mut obstacles: Query<(Entity, &Obstacle, &mut Transform), Without<CameraMarker>>,
    camera: Query<&Transform, (With<CameraMarker>, Without<Obstacle>)>,
    time: Res<Time>,
) {
    for (obstacle_entity, obstacle, mut obstacle_transform) in obstacles.iter_mut() {
        if obstacle_transform.translation.x < camera.single().translation.x - WINWIDTH / 2.0 {
            commands.entity(obstacle_entity).despawn();
        } else {
            obstacle_transform.translation.x += obstacle.speed * time.delta_seconds();
        }
    }
}

fn spawn_new_obstacles(
    mut commands: Commands,
    obstacles: Query<&Transform, (With<Obstacle>, Without<CameraMarker>)>,
    camera: Query<&Transform, (With<CameraMarker>, Without<Obstacle>)>,
    asset_server: Res<AssetServer>,
) {
    if obstacles.iter().count() > 10 {
        return;
    }

    let offset = camera.single().translation.x + WINWIDTH;

    let mut rng = thread_rng();
    let x_pos = rand::random::<f32>() * WINWIDTH + OBSTACLE_WIDTH / 2.0 + offset;
    let parity = (-1.0_f32).powi(rng.gen_range(0..10));
    // let y_pos = parity * rng.gen_range((OBSTACLE_HEIGHT / 2.0 + 20.0)..Y_ABS_RANGE);
    let y_pos = parity * Y_VALUES[rng.gen_range(0..4)];
    let speed = parity * 100.0;

    if obstacles.iter().any(|o| {
        (o.translation.x - x_pos).abs() < OBSTACLE_WIDTH
            && (o.translation.y - y_pos).abs() < OBSTACLE_HEIGHT
    }) {
        return;
    }

    commands.spawn((
        Obstacle { speed },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: OBSTACLE_WIDTH,
                    y: OBSTACLE_HEIGHT,
                }),
                ..default()
            },
            texture: match parity.round() {
                -1.0 => asset_server.load("1082.png"),
                _ => asset_server.load("1081.png"),
            },
            transform: Transform::from_xyz(x_pos, y_pos, 1.0),
            ..default()
        },
    ));
}

fn detect_collision(
    mut commands: Commands,
    mut car: Query<(&mut Car, &Transform), Without<Obstacle>>,
    obstacles: Query<(Entity, &Transform), (With<Obstacle>, Without<Car>)>,
    asset_server: Res<AssetServer>,
) {
    let car_pos = car.single().1.translation;
    for obstacle in obstacles.iter() {
        if (obstacle.1.translation.x - car_pos.x).abs() <= (CARWIDTH + OBSTACLE_WIDTH) / 2.0
            && (obstacle.1.translation.y - car_pos.y).abs() <= (CARHEIGHT + OBSTACLE_HEIGHT) / 2.0
        {
            car.single_mut().0.state = CarState::Crashed;
            commands.entity(obstacle.0).despawn();
            commands.spawn(AudioBundle {
                source: asset_server.load("crash.wav"),
                settings: PlaybackSettings::ONCE,
            });
        }
    }
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("music.mp3"),
        settings: PlaybackSettings::LOOP,
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Midnight Motorist".into(),
                name: Some("Midnight Motorist".into()),
                resolution: (WINWIDTH, WINHEIGHT).into(),
                // present_mode: PresentMode::AutoNoVsync,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, spawn_car)
        .add_systems(Startup, spawn_score)
        .add_systems(Startup, start_music)
        .add_systems(Update, update_background)
        .add_systems(Update, update_car)
        .add_systems(Update, update_obstacles)
        .add_systems(Update, spawn_new_obstacles.after(update_obstacles))
        .add_systems(Update, update_score)
        .add_systems(Update, camera_tracking)
        .add_systems(Update, detect_collision)
        .add_systems(Update, update_laps)
        .run();
}
