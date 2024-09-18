use bevy::prelude::*;
use rand::prelude::*;

use crate::{CameraMarker, Car, CarState, LevelAssets, LevelAssetMarker}; 

const CARHEIGHT: f32 = 105.0;
const CARWIDTH: f32 = 135.0;

#[derive(Component)]
pub struct Obstacle {
    speed: f32,
}

pub fn update_obstacles(
    mut commands: Commands,
    mut obstacles: Query<(Entity, &Obstacle, &mut Transform), Without<CameraMarker>>,
    camera: Query<&Transform, (With<CameraMarker>, Without<Obstacle>)>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let width = window.single().width();
    for (obstacle_entity, obstacle, mut obstacle_transform) in obstacles.iter_mut() {
        if obstacle_transform.translation.x < camera.single().translation.x - width / 2.0 {
            commands.entity(obstacle_entity).despawn();
        } else {
            obstacle_transform.translation.x += obstacle.speed * time.delta_seconds();
        }
    }
}

pub fn spawn_new_obstacles(
    mut commands: Commands,
    obstacles: Query<&Transform, (With<Obstacle>, Without<CameraMarker>)>,
    camera: Query<&Transform, (With<CameraMarker>, Without<Obstacle>)>,
    window: Query<&Window>,
    level_assets: ResMut<LevelAssets>,
) {
    let width = window.single().width();
    let window_scale = 1080.0 / window.single().height();

    if obstacles.iter().count() > 10 {
        return;
    }

    let offset = camera.single().translation.x + width;

    let mut rng = thread_rng();
    let x_pos = rand::random::<f32>() * width + level_assets.obstacle_width / 2.0 + offset;
    let parity = (-1.0_f32).powi(rng.gen_range(0..10));
    let y_pos = parity * level_assets.y_values[rng.gen_range(0..4)] / window_scale;
    let speed = parity * level_assets.obstacle_speed;

    if obstacles.iter().any(|o| {
        (o.translation.x - x_pos).abs() < level_assets.obstacle_width
            && (o.translation.y - y_pos).abs() < level_assets.obstacle_height
    }) {
        return;
    }

    commands.spawn((
        Obstacle { speed },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: level_assets.obstacle_width / window_scale,
                    y: level_assets.obstacle_height / window_scale,
                }),
                ..default()
            },
            texture: match parity.round() {
                -1.0 => level_assets.obstacle_texture[1].clone(),
                _ => level_assets.obstacle_texture[0].clone(),
            },
            transform: Transform::from_xyz(x_pos, y_pos, 1.0),
            ..default()
        },
        LevelAssetMarker
    ));
}

pub fn detect_collision(
    mut commands: Commands,
    mut car: Query<(&mut Car, &Transform), Without<Obstacle>>,
    obstacles: Query<(Entity, &Transform), (With<Obstacle>, Without<Car>)>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
    level_assets: ResMut<LevelAssets>,
) {
    let window_scale = 1080.0 / window.single().height();
    let car_pos = car.single().1.translation;

    for obstacle in obstacles.iter() {
        if (obstacle.1.translation.x - car_pos.x).abs()
            <= 0.95 * ((CARWIDTH + level_assets.obstacle_width) / window_scale) / 2.0
            && (obstacle.1.translation.y - car_pos.y).abs()
                <= 0.90 * ((CARHEIGHT + level_assets.obstacle_height) / window_scale) / 2.0
        {
            car.single_mut().0.state = CarState::Crashed;
            commands.entity(obstacle.0).despawn();
            commands.spawn(AudioBundle {
                source: asset_server.load("crash.wav"),
                settings: PlaybackSettings::DESPAWN,
            });
            car.single_mut().0.collision_counter += 1;
        }
    }
}
