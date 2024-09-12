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
        .insert_state(MenuState::Paused)
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, spawn_ui)
        .add_systems(Startup, spawn_car)
        .add_systems(Startup, spawn_score)
        .add_systems(Startup, spawn_menu)
        .add_systems(Startup, start_music)
        .add_systems(Update, update_menu)
        .add_systems(Update, update_background.run_if(in_state(MenuState::Running)))
        .add_systems(Update, update_score.run_if(in_state(MenuState::Running)))
        .add_systems(Update, camera_tracking.run_if(in_state(MenuState::Running)))
        .add_systems(Update, update_laps.run_if(in_state(MenuState::Running)))
        .add_systems(Update, update_speed.run_if(in_state(MenuState::Running)))
        .add_systems(Update, update_car.run_if(in_state(MenuState::Running)))
        .add_systems(Update, update_obstacles.run_if(in_state(MenuState::Running)))
        .add_systems(Update, spawn_new_obstacles.after(update_obstacles))
        .add_systems(Update, detect_collision.run_if(in_state(MenuState::Running)))
        .run();
}
