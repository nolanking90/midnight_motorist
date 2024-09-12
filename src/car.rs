use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Timer;
use crate::CameraMarker;

const WINSCALE: f32 = 1.5;

const CARHEIGHT: f32 = 105.0 / WINSCALE;
const CARWIDTH: f32 = 135.0 / WINSCALE;
const YSPEED: f32 = 1000.0;

#[derive(Component)]
pub struct Car {
    pub speed: Vec2,
    pub state: CarState,
    pub score: f32,
    sprite_index: usize,
    texture_list: Vec<Handle<Image>>,
    frame_timer: Timer,
    pub position: Vec3,
}

#[derive(PartialEq)]
pub enum CarState {
    Moving,
    Crashed,
}

pub fn spawn_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

pub fn update_car(
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cars: Query<(&mut Car, &mut Transform, &mut Handle<Image>), Without<CameraMarker>>,
    camera: Query<&Transform, With<CameraMarker>>,
    window: Query<&Window>,
) {
    let width = window.single().width();
    let height = window.single().height();

    for (mut car, mut transform, mut sprite) in cars.iter_mut() {
        match car.state {
            CarState::Moving => {
                if button_input.pressed(KeyCode::KeyW) || button_input.pressed(KeyCode::ArrowUp) {
                    car.position.y += car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -height / 2.0 + CARHEIGHT / 2.0,
                        height / 2.0 - CARHEIGHT / 2.0,
                    );
                }
                if button_input.pressed(KeyCode::KeyS) || button_input.pressed(KeyCode::ArrowDown) {
                    car.position.y -= car.speed.y * time.delta_seconds();
                    car.position.y = car.position.y.clamp(
                        -height / 2.0 + CARHEIGHT / 2.0,
                        height / 2.0 - CARHEIGHT / 2.0,
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
