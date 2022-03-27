use crate::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(ScreenState::Play).with_system(setup))
            .add_system_set(SystemSet::on_update(ScreenState::Play).with_system(update))
            .add_system_set(SystemSet::on_exit(ScreenState::Play).with_system(teardown));
    }
}

fn setup(commands: Commands, asset_server: Res<AssetServer>) {
    println!("GAME");
}

fn update(mut state: ResMut<State<ScreenState>>, mut keyboard_input: ResMut<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        keyboard_input.clear_just_pressed(KeyCode::Escape);
        state.set(ScreenState::Menu).unwrap();
    }
}
