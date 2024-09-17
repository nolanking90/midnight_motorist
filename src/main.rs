use bevy::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod ui;
use ui::*;

mod menu;
use menu::*;

mod car;
use car::*;

mod obstacle;
use obstacle::*;

mod levels;
use levels::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Midnight Motorist".into(),
                name: Some("Midnight Motorist".into()),
                resolution: (1280.0, 720.0).into(),
                // present_mode: PresentMode::AutoNoVsync,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        //.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, spawn_ui)
        .add_systems(Startup, spawn_score)
        .add_systems(Startup, spawn_camera)
        //.add_systems(Startup, spawn_menu)
        .add_systems(Update, update_menu)
        .add_systems(Update, update_score.run_if(in_state(GameState::Running)))
        .add_systems(Update, camera_tracking.run_if(in_state(GameState::Running)))
        .add_systems(Update, update_laps.run_if(in_state(GameState::Running)))
        .add_systems(Update, update_speed.run_if(in_state(GameState::Running)))

        .insert_resource(Level { level: 1 })
        .insert_resource(LevelAssets { ..default() })
        .insert_state(GameState::LoadNextLevel)
        .add_systems(Startup, load_level.after(spawn_camera))
        .add_systems(Startup, (spawn_background, spawn_car, start_music).after(load_level))
        .add_systems(Update, game_over.run_if(in_state(GameState::Running)))
        .add_systems(Update, next_level.run_if(in_state(GameState::Running)))
        .add_systems(Update, despawn_level.run_if(in_state(GameState::LoadNextLevel)))
        .add_systems(Update, load_level.run_if(in_state(GameState::LoadNextLevel)).after(despawn_level))
        .add_systems(PostUpdate, (spawn_car, spawn_background).after(load_level))

        .add_systems(Update, update_background.run_if(in_state(GameState::Running)))
        .add_systems(Update, update_car.run_if(in_state(GameState::Running)))
        .add_systems(Update, update_obstacles.run_if(in_state(GameState::Running)))
        .add_systems(Update, spawn_new_obstacles.after(update_obstacles))
        .add_systems(Update, detect_collision.run_if(in_state(GameState::Running)))
        .run();
}
