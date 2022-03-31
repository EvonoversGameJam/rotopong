use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct InputConfig {
    pub left: KeyCode,
    pub right: KeyCode,
    pub down: KeyCode,
    pub up: KeyCode,
    pub rot_1: KeyCode,
    pub rot_2: KeyCode,
}

impl InputConfig {
    pub fn left() -> Self {
        Self {
            right: KeyCode::D,
            left: KeyCode::A,
            up: KeyCode::W,
            down: KeyCode::S,
            rot_1: KeyCode::Q,
            rot_2: KeyCode::E,
        }
    }

    pub fn right() -> Self {
        Self {
            right: KeyCode::Right,
            left: KeyCode::Left,
            up: KeyCode::Up,
            down: KeyCode::Down,
            rot_1: KeyCode::Comma,
            rot_2: KeyCode::Period,
        }
    }
}
