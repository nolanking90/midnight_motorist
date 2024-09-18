use crate::CameraMarker;
use crate::{car::*, menu::*, Background, Obstacle};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Level {
    pub level: u8,
}

//fn read_resource_system(resource: Res<MyResource>) {
//assert_eq!(resource.value, 42);
//}

//fn write_resource_system(mut resource: ResMut<MyResource>) {
//assert_eq!(resource.value, 42);
//resource.value = 0;
//assert_eq!(resource.value, 0);
//}

#[derive(Resource, Default)]
pub struct LevelAssets {
    pub car_texture: Handle<Image>,
    pub obstacle_texture: Vec<Handle<Image>>,
    pub obstacle_height: f32,
    pub obstacle_width: f32,
    pub obstacle_speed: f32,
    pub y_values: [f32; 4],
    pub background_texture: Handle<Image>,
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
    camera: Query<&Transform, With<CameraMarker>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level: ResMut<Level>,
) {
    let width = window.single().width();
    let laps = (camera.single().translation.x / (width) / 10.0) as u8;

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
        println!("Level: {}", level.level);
        next_state.set(GameState::LoadNextLevel);
    }
}

pub fn despawn_level(
    mut commands: Commands,
    old_obstacles: Query<
        Entity,
        (
            With<Obstacle>,
            Without<Background>,
            Without<Car>,
            Without<MenuText>,
        ),
    >,
    old_backgrounds: Query<
        Entity,
        (
            With<Background>,
            Without<Obstacle>,
            Without<Car>,
            Without<MenuText>,
        ),
    >,
    old_car: Query<
        Entity,
        (
            With<Car>,
            Without<Obstacle>,
            Without<Background>,
            Without<MenuText>,
        ),
    >,
    menutext: Query<
        Entity,
        (
            With<MenuText>,
            Without<Obstacle>,
            Without<Background>,
            Without<Car>,
        ),
    >,
) {
    if !old_obstacles.is_empty() {
        old_obstacles
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
    }
    if !old_car.is_empty() {
        old_car
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
    }
    if !old_backgrounds.is_empty() {
        old_backgrounds
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
    }

    if !menutext.is_empty() {
        commands.entity(menutext.single()).despawn();
    }
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
            };
        }
        3 => {}
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
    menutext: Query<Entity, With<MenuText>>
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
    next_state.set(GameState::Running);
}
