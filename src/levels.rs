use std::time::Duration;

use crate::CameraMarker;
use crate::{car::*, menu::*};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Level {
    pub level: u8,
}

#[derive(Component)]
pub struct LevelAssetMarker;

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub car_texture: Handle<Image>,
    pub obstacle_texture: Vec<Handle<Image>>,
    pub obstacle_height: f32,
    pub obstacle_width: f32,
    pub obstacle_speed: f32,
    pub y_values: [f32; 4],
    pub background_texture: Handle<Image>,
    pub music: Handle<AudioSource>,
    pub lap_texture: Handle<Image>,
}

pub fn game_over(
    mut commands: Commands,
    car: Query<&Car>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if car.is_empty() {
        return;
    }
    if car.single().collision_counter == 5 {
        commands.spawn((
            TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(42.0),
                height: Val::Percent(16.0),
                left: Val::Percent(25.0),
                width: Val::Percent(50.0),
                ..default()
            }),
            MenuText,
        ));
        next_state.set(GameState::Paused);
    }
}

pub fn next_level(
    mut commands: Commands,
    window: Query<&Window>,
    car: Query<&Transform, With<Car>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level: ResMut<Level>,
) {
    let width = window.single().width();
    let laps = (car.single().translation.x / (width) / 10.0) as u8;

    if laps == 1 {
        commands.spawn((
            TextBundle::from_section(
                "SUCCESS",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(42.0),
                height: Val::Percent(16.0),
                left: Val::Percent(25.0),
                width: Val::Percent(50.0),
                ..default()
            }),
            MenuText,
        ));
        level.level += 1;
        next_state.set(GameState::Unloading);
    }
}

pub fn despawn_level(
    mut commands: Commands,
    old_assets: Query<Entity, (With<LevelAssetMarker>, Without<MenuText>)>,
    menutext: Query<Entity, (With<MenuText>, Without<LevelAssetMarker>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !old_assets.is_empty() {
        old_assets
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
    }

    if !menutext.is_empty() {
        commands.entity(menutext.single()).despawn();
    }

    next_state.set(GameState::LoadNextLevel);
}

pub fn load_level(
    level: Res<Level>,
    mut assets: ResMut<LevelAssets>,
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<CameraMarker>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    camera.single_mut().translation = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    match level.level {
        1 => {
            *assets = LevelAssets {
                obstacle_height: 105.0,
                obstacle_width: 135.0,
                obstacle_speed: 100.0,
                obstacle_texture: vec![
                    asset_server.load("1081.png"),
                    asset_server.load("1082.png"),
                ],
                y_values: [
                    (105.0 / 2.0 + 20.0),
                    (135.0 + 105.0 / 2.0 + 10.0),
                    (2.0 * 135.0 + 105.0 / 2.0 + 15.0),
                    (3.0 * 135.0 + 105.0 / 2.0 - 10.0),
                ],
                car_texture: asset_server.load("1084.png"),
                background_texture: asset_server.load("1058.png"),
                music: asset_server.load("240bps.mp3"),
                lap_texture: asset_server.load("1077.png"),
            };
        }
        2 => {
            *assets = LevelAssets {
                obstacle_height: 291.0,
                obstacle_width: 202.0,
                obstacle_speed: 0.0,
                obstacle_texture: vec![
                    asset_server.load("1149.png"),
                    asset_server.load("1149.png"),
                ],
                y_values: [
                    (105.0 / 2.0 + 20.0),
                    (135.0 + 105.0 / 2.0 + 10.0),
                    (2.0 * 135.0 + 105.0 / 2.0 + 15.0),
                    (3.0 * 135.0 + 105.0 / 2.0 - 10.0),
                ],
                car_texture: asset_server.load("1145.png"),
                background_texture: asset_server.load("backroads.png"),
                music: asset_server.load("dui.mp3"),
                lap_texture: asset_server.load("1077.png"),
            };
        }
        3 => {
            *assets = LevelAssets {
                obstacle_height: 105.0,
                obstacle_width: 135.0,
                obstacle_speed: 0.0,
                obstacle_texture: vec![
                    asset_server.load("1081.png"),
                    asset_server.load("1082.png"),
                ],
                y_values: [
                    (105.0 / 2.0 + 20.0),
                    (135.0 + 105.0 / 2.0 + 10.0),
                    (2.0 * 135.0 + 105.0 / 2.0 + 15.0),
                    (3.0 * 135.0 + 105.0 / 2.0 - 10.0),
                ],
                car_texture: asset_server.load("tank.png"),
                background_texture: asset_server.load("rainbowroad.png"),
                music: asset_server.load("dui.mp3"),
                lap_texture: asset_server.load("1077.png"),
            };
        }
        _ => {}
    }
    next_state.set(GameState::Loading);
}

pub fn spawn_loading_screen(mut commands: Commands, menu_text: Query<&MenuText>) {
    if !menu_text.is_empty() {
        return;
    }
    commands.spawn((
        TextBundle::from_section(
            "Loading",
            TextStyle {
                font_size: 100.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(42.0),
            height: Val::Percent(16.0),
            left: Val::Percent(25.0),
            width: Val::Percent(50.0),
            ..default()
        }),
        MenuText,
    ));
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_assets: Res<LevelAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    menutext: Query<Entity, With<MenuText>>,
) {
    if asset_server
        .get_load_state(&level_assets.car_texture)
        .unwrap()
        != bevy::asset::LoadState::Loaded
    {
        return;
    }
    if asset_server
        .get_load_state(&level_assets.obstacle_texture[0])
        .unwrap()
        != bevy::asset::LoadState::Loaded
    {
        return;
    }
    if asset_server
        .get_load_state(&level_assets.obstacle_texture[1])
        .unwrap()
        != bevy::asset::LoadState::Loaded
    {
        return;
    }
    if asset_server
        .get_load_state(&level_assets.background_texture)
        .unwrap()
        != bevy::asset::LoadState::Loaded
    {
        return;
    }

    if !menutext.is_empty() {
        commands.entity(menutext.single()).despawn();
    }
    next_state.set(GameState::Countdown);
}

#[derive(Component)]
pub struct Countdown {
    pub frame_timer: Timer,
    pub index: usize,
}

#[derive(Resource, Default)]
pub struct CountdownAssets {
    pub images: [Handle<Image>; 4],
    pub sound: Handle<AudioSource>,
    pub go_sound: Handle<AudioSource>,
}

pub fn spawn_countdown_assets(
    mut countdown_assets: ResMut<CountdownAssets>,
    asset_server: ResMut<AssetServer>,
) {
    countdown_assets.images = [
        asset_server.load("cd1.png"),
        asset_server.load("cd2.png"),
        asset_server.load("cd3.png"),
        asset_server.load("go.png"),
    ];
    countdown_assets.sound = asset_server.load("countdown.wav");
    countdown_assets.go_sound = asset_server.load("go3.wav");
}

pub fn start_countdown(
    mut commands: Commands,
    countdown_assets: Res<CountdownAssets>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timer: Query<(&mut Countdown, &mut UiImage, Entity)>,
) {
    if timer.is_empty() {
        commands.spawn((
            Countdown {
                frame_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
                index: 0,
            },
            ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::horizontal(Val::Auto),
                    top: Val::Percent(25.0),
                    left: Val::Percent(33.0),
                    width: Val::Percent(34.0),
                    height: Val::Percent(50.0),
                    ..Default::default()
                },
                image: UiImage {
                    texture: countdown_assets.images[0].clone(),
                    ..default()
                },
                ..default()
            },
        ));
        commands.spawn(AudioBundle {
            source: countdown_assets.sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        return;
    }

    if timer.single().0.frame_timer.paused() {
        timer.single_mut().0.frame_timer.unpause();
    }

    timer.single_mut().0.frame_timer.tick(time.delta());

    if timer.single().0.frame_timer.just_finished() {
        if timer.single().0.index == 3 {
            commands.entity(timer.single().2).despawn();
            next_state.set(GameState::Running);
            return;
        }

        timer.single_mut().0.index += 1;
        timer.single_mut().1.texture = countdown_assets.images[timer.single().0.index].clone();
        commands.spawn(AudioBundle {
            source: match timer.single().0.index {
                3 => countdown_assets.go_sound.clone(),
                _ => countdown_assets.sound.clone(),
            },
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
