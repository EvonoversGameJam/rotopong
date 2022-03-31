mod components;
use components::*;

use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(apply_velocity)
        .add_system(revert_velocity)
        .add_system(move_paddle)
        .add_system(rotate_paddle)
        .add_system(check_collisions)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(Ball)
        .insert(Velocity(Vec3::new(5., 1., 0.).normalize()))
        .insert(Speed(50.))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        });

    let window = windows.get_primary().unwrap();
    // commands
    //     .spawn()
    //     .insert(Paddle)
    //     .insert(InputConfig::left())
    //     .insert(Velocity(Vec3::ZERO))
    //     .insert(Speed(100.))
    //     .insert_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(100.0, 10.0)),
    //             ..Default::default()
    //         },
    //         transform: Transform {
    //             rotation: Quat::from_rotation_z(PI * 3.0 / 2.0),
    //             translation: Vec3::new(-window.width() / 2. + 10., 0., 0.),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });
    commands
        .spawn()
        .insert(Paddle)
        .insert(InputConfig::right())
        .insert(Velocity(Vec3::ZERO))
        .insert(Speed(100.))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 10.0)),
                ..Default::default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                translation: Vec3::new(window.width() / 2. - 10., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity, &Speed)>) {
    for (mut transform, velocity, speed) in query.iter_mut() {
        transform.translation += velocity.0 * speed.0 * time.delta().as_secs_f32();
    }
}

fn revert_velocity(
    windows: Res<Windows>,
    mut query: Query<(&mut Velocity, &Transform), With<Ball>>,
) {
    let (mut velocity, transform) = query.single_mut();

    let window = windows.get_primary().unwrap();
    if transform.translation.x > window.width() / 2.
        || transform.translation.x < -window.width() / 2.
    {
        velocity.0.x *= -1.;
    }

    if transform.translation.y > window.height() / 2.
        || transform.translation.y < -window.height() / 2.
    {
        velocity.0.y *= -1.;
    }
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&InputConfig, &mut Velocity, &Transform), With<Paddle>>,
) {
    for (keys, mut velocity, transform) in query.iter_mut() {
        let mut res_velocity = Vec3::ZERO;
        if input.pressed(keys.right) {
            res_velocity += transform.right();
        }
        if input.pressed(keys.up) {
            res_velocity += transform.up();
        }
        if input.pressed(keys.left) {
            res_velocity += transform.left();
        }
        if input.pressed(keys.down) {
            res_velocity += transform.down();
        }

        velocity.0 = if res_velocity == Vec3::ZERO {
            res_velocity
        } else {
            res_velocity.normalize()
        };
    }
}

fn check_collisions(
    mut ball: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    let (mut ball_velocity, ball_transform) = ball.single_mut();
    for paddle_transform in paddles.iter() {
        let result = ball_transform.translation - paddle_transform.translation;
        let rotated = paddle_transform.rotation * result;
        if rotated.x.abs() < 50. && rotated.y.abs() < 5. {
            ball_velocity.0 = result.normalize();
        }
    }
}

fn rotate_paddle(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &InputConfig), With<Paddle>>,
) {
    for (mut transform, keys) in query.iter_mut() {
        if input.pressed(keys.rot_1) {
            transform.rotation *= Quat::from_rotation_z(time.delta().as_secs_f32() * 2.);
        }
        if input.pressed(keys.rot_2) {
            transform.rotation *= Quat::from_rotation_z(time.delta().as_secs_f32() * -2.);
        }
    }
}
