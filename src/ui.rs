use bevy::{prelude::*, time::Time};

use crate::Car ;

#[derive(Component)]
pub struct CameraMarker;


pub fn camera_tracking(
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<CameraMarker>>,
    player: Query<&Car>,
) {
    camera.single_mut().translation.x += player.single().speed.x * time.delta_seconds();
}

#[derive(Component)]
pub struct Score {
    digits: Vec<Handle<Image>>,
}

#[derive(Component)]
pub struct ScoreDigit;

#[derive(Component)]
pub struct SpeedDigit;

pub fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
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


#[derive(Component)]
pub struct LapsDigit;

pub fn update_laps(
    mut commands: Commands,
    camera: Query<&Transform, With<CameraMarker>>,
    score: Query<&Score>,
    window: Query<&Window>,
    prev_laps_digit: Query<Entity, With<LapsDigit>>,
) {
    let width = window.single().width();
    let digit = camera.single().translation.x / (width) / 10.0;

    for prev_digit in prev_laps_digit.iter() {
        commands.entity(prev_digit).despawn();
    }

    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect::horizontal(Val::Auto),
                top: Val::Percent(50.0 / 1080.0 * 100.0),
                left: Val::Percent(7.5),
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
        LapsDigit
    ));
}

pub fn update_score(
    mut commands: Commands,
    cars: Query<&Car>,
    score: Query<&Score>,
    prev_score_digits: Query<Entity, With<ScoreDigit>>,
    prev_speed_digits: Query<Entity, With<SpeedDigit>>,
    window: Query<&Window>,
) {
    for prev_digit in prev_score_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let width = window.single().width();
    let player_score = cars.single().score.floor() as u32;
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
                    texture: score.single().digits[digit as usize].clone(),
                    ..default()
                },
                ..Default::default()
            },
            ScoreDigit,
        ));
        left_pos += 0.03 * width;
    }

    for prev_digit in prev_speed_digits.iter() {
        commands.entity(prev_digit).despawn();
    }

    let player_speed = (cars.single().speed.x / 10.0).floor() as u32;
    let speed_string = player_speed.to_string();
    let temp = speed_string.chars();
    let mut left_pos = 20.0 + 0.06 * width;

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
        left_pos += 0.03 * width;
    }
}

pub fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("music.mp3"),
        settings: PlaybackSettings::LOOP,
    });
}
