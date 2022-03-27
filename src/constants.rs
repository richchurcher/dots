use bevy::prelude::Color;

pub const BACKGROUND_COLOR: Color = Color::rgb(38. / 255., 20. / 255., 40. / 255.);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScreenState {
    Menu,
    Play,
}

#[derive(Default)]
pub struct GameState {
    pub spots: u64,
}
