use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod ui;
use ui::*;

mod car;
use car::*;

mod obstacle;
use obstacle::*;

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
struct Background;

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
