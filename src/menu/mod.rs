use crate::*;
use bevy::app::AppExit;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ScreenState::Menu).with_system(setup))
            .add_system_set(SystemSet::on_update(ScreenState::Menu).with_system(update))
            .add_system_set(SystemSet::on_exit(ScreenState::Menu).with_system(teardown));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(4.0),
                left: Val::Px(24.0),
                ..default()
            },
            ..default()
        },
        text: Text::with_section(
            format!("dots"),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            },
        ),
        ..default()
    });
}

fn update(
    mut state: ResMut<State<ScreenState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(ScreenState::Play).unwrap();
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
