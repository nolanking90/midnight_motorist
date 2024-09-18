use std::f32::consts::PI;
use std::time::Duration;

use crate::{CameraMarker, LevelAssets, Score};
use bevy::prelude::*;
use bevy::time::Timer;

const CARHEIGHT: f32 = 105.0;
const CARWIDTH: f32 = 135.0;
const YSPEED: f32 = 500.0;

#[derive(Component)]
pub struct Car {
    pub speed: Vec2,
    pub state: CarState,
    pub score: f32,
    sprite_index: usize,
    frame_timer: Timer,
    pub position: Vec3,
    pub collision_counter: u8,
}

#[derive(PartialEq)]
pub enum CarState {
    Moving,
    Crashed,
}

pub fn spawn_car(
    mut commands: Commands,
    window: Query<&Window>,
    level_assets: ResMut<LevelAssets>,
    car: Query<&Car>,
    score: Res<Score>
) {
    if !car.is_empty() {
        return;
    }

    let window_scale = 1080.0 / window.single().height();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: CARWIDTH / window_scale,
                    y: CARHEIGHT / window_scale,
                }),
                ..default()
            },
            texture: level_assets.car_texture.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Car {
            speed: Vec2 { x: 0.0, y: YSPEED },
            score: score.score,
            state: CarState::Moving,
            frame_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
            sprite_index: 0,
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            collision_counter: 0,
        },
    ));
}

pub fn update_car(
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cars: Query<(&mut Car, &mut Transform), Without<CameraMarker>>,
    camera: Query<&Transform, With<CameraMarker>>,
    window: Query<&Window>,
) {
    let width = window.single().width();
    let height = window.single().height();
    let window_scale = 1080.0 / height;

    for (mut car, mut transform) in cars.iter_mut() {
        match car.state {
            CarState::Moving => {
                if button_input.pressed(KeyCode::KeyW) || button_input.pressed(KeyCode::ArrowUp) {
                    car.position.y += car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -height / 2.0 + CARHEIGHT / window_scale / 2.0,
                        height / 2.0 - CARHEIGHT / window_scale / 2.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyS) || button_input.pressed(KeyCode::ArrowDown) {
                    car.position.y -= car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -height / 2.0 + CARHEIGHT / window_scale / 2.0,
                        height / 2.0 - CARHEIGHT / window_scale / 2.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyD) || button_input.pressed(KeyCode::ArrowRight)
                {
                    car.position.x += car.speed.y * time.delta_seconds();
                    car.position.x = car.position.x.clamp(
                        camera.single().translation.x - width / 4.0,
                        camera.single().translation.x + width / 4.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyA) || button_input.pressed(KeyCode::ArrowLeft) {
                    car.position.x -= car.speed.y * time.delta_seconds();
                    car.position.x = car.position.x.clamp(
                        camera.single().translation.x - width / 4.0,
                        camera.single().translation.x + width / 4.0,
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

                transform.rotation = transform.rotation.mul_quat(Quat::from_axis_angle(
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    2.0 * PI / 20.0,
                ));

                if car.frame_timer.just_finished() {
                    car.sprite_index = 0;
                    car.state = CarState::Moving;
                    transform.rotation = Quat::from_axis_angle(
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        0.0,
                    );
                }
            }
        }
    }
}
