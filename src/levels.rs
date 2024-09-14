use crate::car::*;
use crate::menu::*;
use crate::CameraMarker;
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

#[derive(Component)]
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
) {
    let width = window.single().width();
    let laps = (camera.single().translation.x / (width) / 10.0) as u8;

    if laps == 5 {
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
        next_state.set(GameState::LoadNextLevel);
    }
}

pub fn load_level(
    mut commands: Commands,
    level: Res<Level>,
    level_assets: Query<Entity, With<LevelAssets>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    level_assets.iter().for_each(|lvl| { commands.entity(lvl).despawn(); });
    match level.level {
        1 => {
            commands.spawn(LevelAssets {
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
            });
        }
        2 => {}
        3 => {}
        _ => {}
    }
    next_state.set(GameState::Running);
}
