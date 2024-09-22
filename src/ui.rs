use bevy::{prelude::*, time::Time};

use crate::{Car, LevelAssetMarker, LevelAssets};

#[derive(Component)]
pub struct CameraMarker;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CameraMarker));
}

pub fn camera_tracking(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<CameraMarker>>,
    player: Query<&Car>,
) {
    if camera.is_empty() || player.is_empty() {
        return;
    }
    camera.single_mut().translation.x += player.single().speed.x * time.delta_seconds();
}

#[derive(Component)]
pub struct Background;

pub fn spawn_background(
    mut commands: Commands,
    window: Query<&Window>,
    level_assets: ResMut<LevelAssets>,
    asset_server: ResMut<AssetServer>,
) {
    let width = window.single().width();
    let height = window.single().height();

    commands.spawn((
        SpriteBundle {
            texture: level_assets.background_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: width * 3.0,
                    y: height,
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
            stretch_value: height / 1080.0,
        },
        Background,
        LevelAssetMarker,
    ));
    commands.spawn((
        SpriteBundle {
            texture: level_assets.background_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: width,
                    y: height,
                }),
                anchor: bevy::sprite::Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_xyz(width * -1.0, 0.0, 0.0),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: false,
            tile_y: false,
            stretch_value: height / 1080.0,
        },
        Background,
        LevelAssetMarker,
    ));
    for n in 1..5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("1077.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 64.0, y: height }),
                    anchor: bevy::sprite::Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_xyz((n as f32) * width * 10.0, 0.0, 2.0),
                ..default()
            },
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
        ));
    }
}

pub fn update_background(
    mut commands: Commands,
    camera: Query<&Transform, (With<CameraMarker>, Without<Background>)>,
    backgrounds: Query<(Entity, &Transform), (With<Background>, Without<CameraMarker>)>,
    window: Query<&Window>,
    level_assets: Res<LevelAssets>,
) {
    let width = window.single().width();
    let height = window.single().height();

    for (background_entity, background_transform) in backgrounds.iter() {
        if background_transform.translation.x < camera.single().translation.x - width * 4.0 {
            commands.entity(background_entity).despawn();
        }
    }
    let road_remaining = backgrounds.iter().any(|(_, background_transform)| {
        background_transform.translation.x > camera.single().translation.x
    });
    if !road_remaining {
        let road_length_factor = (camera.single().translation.x / (width * 3.0)).floor() + 1.0;

        commands.spawn((
            SpriteBundle {
                texture: level_assets.background_texture.clone(), // Background
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: width * 3.0,
                        y: height,
                    }),
                    anchor: bevy::sprite::Anchor::CenterLeft,
                    ..default()
                },
                transform: Transform::from_xyz(road_length_factor * 3.0 * width, 0.0, 0.0),
                ..default()
            },
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: height / 1080.0,
            },
            Background,
            LevelAssetMarker,
        ));
    }
}

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "LAPS",
            TextStyle {
                font_size: 45.0,
                font: asset_server.load("GohuFont11NerdFont-Regular.ttf"),
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(50.0 / 1080.0 * 100.0),
            left: Val::Percent(10.0 / 1920.0 * 100.0),
            width: Val::Percent(5.0),
            height: Val::Percent(5.0),
            ..Default::default()
        }),
    );
    commands.insert_resource(Lap {
        lap: 0,
        font: asset_server.load("GohuFont11NerdFont-Regular.ttf"),
    });

    commands.spawn(
        TextBundle::from_section(
            "MPH",
            TextStyle {
                font_size: 45.0,
                font: asset_server.load("GohuFont11NerdFont-Regular.ttf"),
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(115.0 / 1080.0 * 100.0),
            left: Val::Percent(10.0 / 1920.0 * 100.0),
            width: Val::Percent(6.0),
            height: Val::Percent(5.0),
            ..Default::default()
        }),
    );
}

#[derive(Resource)]
pub struct Score {
    digits: Vec<Handle<Image>>,
    pub score: f32,
}

#[derive(Component)]
pub struct ScoreDigit;

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut digits: Vec<Handle<Image>> = Vec::new();
    for n in 0..10 {
        digits.push(asset_server.load((1687 + n).to_string() + ".png"));
    }
    commands.insert_resource(Score { digits, score: 0.0 });
}

pub fn update_score(
    mut commands: Commands,
    cars: Query<&Car>,
    score: Res<Score>,
    prev_score_digits: Query<Entity, With<ScoreDigit>>,
    window: Query<&Window>,
) {
    if cars.is_empty() {
        return;
    }

    for prev_digit in prev_score_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let width = window.single().width();
    let player_score = score.score.floor() as u32;
    let score_string = player_score.to_string();
    let temp = score_string.chars();
    let num_digits = temp.clone().count();
    let mut left_pos = width - 10.0 - (0.03 * width * num_digits as f32);

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
                    texture: score.digits[digit as usize].clone(),
                    ..default()
                },
                ..Default::default()
            },
            ScoreDigit,
        ));
        left_pos += 0.03 * width;
    }
}

#[derive(Component)]
pub struct LapsDigit;

#[derive(Resource)]
pub struct Lap {
    pub lap: usize,
    pub font: Handle<Font>,
}

pub fn update_laps(
    mut commands: Commands,
    car: Query<&Transform, With<Car>>,
    mut lap: ResMut<Lap>,
    window: Query<&Window>,
    prev_laps_digit: Query<Entity, With<LapsDigit>>,
) {
    let width = window.single().width();
    let digit = car.single().translation.x / width / 10.0;

    if digit as usize > lap.lap || prev_laps_digit.is_empty() {
        lap.lap = digit as usize;

        for prev_digit in prev_laps_digit.iter() {
            commands.entity(prev_digit).despawn();
        }

        commands.spawn((
            TextBundle::from_section(
                lap.lap.clone().to_string(),
                TextStyle {
                    font_size: 60.0,
                    font: lap.font.clone(),
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                margin: UiRect::horizontal(Val::Auto),
                top: Val::Percent(3.5),
                left: Val::Percent(8.0),
                width: Val::Percent(3.0),
                height: Val::Percent(5.0),
                ..Default::default()
            }),
            LapsDigit,
            LevelAssetMarker,
        ));
    }
}

#[derive(Component)]
pub struct SpeedDigit;

pub fn update_speed(
    mut commands: Commands,
    cars: Query<&Car>,
    lap: Res<Lap>,
    prev_speed_digits: Query<Entity, With<SpeedDigit>>,
) {
    for prev_digit in prev_speed_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let mut player_speed = 0;
    if !cars.is_empty() {
        player_speed = (cars.single().speed.x / 10.0).floor() as u32;
    }

    commands.spawn((
        TextBundle::from_section(
            player_speed.to_string(),
            TextStyle {
                font_size: 50.0,
                font: lap.font.clone(),
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Percent(10.0),
            left: Val::Percent(7.0),
            width: Val::Percent(3.0),
            height: Val::Percent(5.0),
            ..Default::default()
        }),
        SpeedDigit,
        LevelAssetMarker,
    ));
}

#[derive(Component)]
pub struct MusicMarker;

pub fn start_music(mut commands: Commands, assets: Res<LevelAssets>, music: Query<&MusicMarker>) {
    if music.is_empty() {
        commands.spawn((
            AudioBundle {
                source: assets.music.clone(),
                settings: PlaybackSettings::LOOP,
            },
            MusicMarker,
            LevelAssetMarker,
        ));
    }
}
