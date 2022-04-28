use std::f32::consts::PI;

use bevy::prelude::*;
use impacted::CollisionShape;

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: Paddle,
    keys: InputConfig,
    velocity: Velocity,
    speed: Speed,
    collision: CollisionShape,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(keys: InputConfig, transform: Transform) -> Self {
        Self {
            tag: Paddle,
            keys,
            velocity: Velocity(Vec3::ZERO),
            speed: Speed(500.),
            collision: CollisionShape::new_rectangle(100.0, 10.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 10.0)),
                    color: Color::CYAN,
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        }
    }

    pub fn spawn_left(commands: &mut Commands, window: &Window) {
        commands.spawn().insert_bundle(PlayerBundle::new(
            InputConfig::left(),
            Transform {
                rotation: Quat::from_rotation_z(PI * 3.0 / 2.0),
                translation: Vec3::new(-window.width() / 2. + 30., 0., 0.),
                ..Default::default()
            },
        ));
    }

    pub fn spawn_right(commands: &mut Commands, window: &Window) {
        commands.spawn().insert_bundle(PlayerBundle::new(
            InputConfig::right(),
            Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3::new(window.width() / 2. - 30., 0., 0.),
                ..Default::default()
            },
        ));
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    tag: Ball,
    velocity: Velocity,
    speed: Speed,
    collision: CollisionShape,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl BallBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            tag: Ball,
            velocity: Velocity(Vec3::X),
            speed: Speed(500.),
            collision: CollisionShape::new_rectangle(10.0, 10.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        }
    }

    pub fn spawn(commands: &mut Commands) {
        commands.spawn().insert_bundle(BallBundle::new(Transform {
            rotation: Quat::from_rotation_z(PI / 4.0),
            ..Default::default()
        }));
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    tag: Wall,
    collision: CollisionShape,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    pub fn new(tag: Wall, transform: Transform, size: Vec2) -> Self {
        Self {
            tag,
            collision: CollisionShape::new_rectangle(size.x, size.y),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        }
    }

    pub fn spawn(commands: &mut Commands, window: &Window) {
        commands.spawn().insert_bundle(WallBundle::new(
            Wall::Vertical(Side::Left),
            Transform {
                translation: Vec3::new(-window.width() / 2. + 10.0, 0.0, 0.0),
                ..Default::default()
            },
            Vec2::new(10.0, window.height() - 10.),
        ));
        commands.spawn().insert_bundle(WallBundle::new(
            Wall::Vertical(Side::Right),
            Transform {
                translation: Vec3::new(window.width() / 2. - 10.0, 0.0, 0.0),
                ..Default::default()
            },
            Vec2::new(10.0, window.height() - 10.),
        ));
        commands.spawn().insert_bundle(WallBundle::new(
            Wall::Horizontal(HSide::Top),
            Transform {
                translation: Vec3::new(0.0, -window.height() / 2. + 10.0, 0.0),
                ..Default::default()
            },
            Vec2::new(window.width() - 10., 10.0),
        ));
        commands.spawn().insert_bundle(WallBundle::new(
            Wall::Horizontal(HSide::Bottom),
            Transform {
                translation: Vec3::new(0.0, window.height() / 2. - 10.0, 0.0),
                ..Default::default()
            },
            Vec2::new(window.width() - 10., 10.0),
        ));
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Paddle;

pub enum Side {
    Left,
    Right
}

pub enum HSide {
    Top,
    Bottom
}

#[derive(Component)]
pub enum Wall {
    Horizontal(HSide),
    Vertical(Side),
}

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
