use bevy::prelude::{Color, Component};

#[derive(Component)]
pub struct Dot {
    pub name: String,
    pub color: Color,
    pub position: Position,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
