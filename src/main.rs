use bevy::prelude::*;

mod constants;
use constants::*;

mod menu;
use menu::MenuPlugin;

mod game;
use game::GamePlugin;

mod components;
use components::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "dots".to_string(),
            ..default()
        })
        .insert_resource(GameState { spots: 0 })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_state(ScreenState::Menu);

    app.add_startup_system(camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
