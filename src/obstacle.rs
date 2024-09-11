use bevy::prelude::*;
use rand::prelude::*;

use crate::{CameraMarker, Car, CarState, CARHEIGHT, CARWIDTH, OBSTACLE_HEIGHT, OBSTACLE_WIDTH, WINWIDTH, Y_VALUES};

#[derive(Component)]
pub struct Obstacle {
    speed: f32,
}

pub fn update_obstacles(
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

pub fn spawn_new_obstacles(
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

pub fn detect_collision(
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
