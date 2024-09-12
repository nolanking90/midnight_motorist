use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    Running,
    Paused,
}

#[derive(Component)]
pub struct MenuText;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "PAUSED",
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
}

pub fn update_menu(
    mut commands: Commands,
    state: Res<State<MenuState>>,
    button_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MenuState>>,
    menu_text: Query<Entity, With<MenuText>>,
) {
    if button_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            MenuState::Running => {
                commands.spawn((
                    TextBundle::from_section(
                        "PAUSED",
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
                next_state.set(MenuState::Paused);
            }
            MenuState::Paused => {
                menu_text.iter().for_each(|text| {
                    commands.entity(text).despawn();
                });
                next_state.set(MenuState::Running);
            }
        }
    }
}
