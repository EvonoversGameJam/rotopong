use std::f32::consts::PI;

use bevy::{prelude::*, transform::TransformSystem};
use impacted::CollisionShape;
// use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
use components::*;

#[derive(Debug)]
struct Score {
    pub left: usize,
    pub right: usize,
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(Score { left: 0, right: 0 })
        .add_system(apply_velocity)
        .add_system(move_paddle)
        .add_system(rotate_paddle)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            update_shape_transforms
                .chain(check_collisions)
                .after(TransformSystem::TransformPropagate),
        )
        .add_plugins(DefaultPlugins)
        // Inspector
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary().unwrap();

    BallBundle::spawn(&mut commands);
    WallBundle::spawn(&mut commands, window);
    PlayerBundle::spawn_left(&mut commands, window);
    PlayerBundle::spawn_right(&mut commands, window);
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity, &Speed)>) {
    for (mut transform, velocity, speed) in query.iter_mut() {
        transform.translation += velocity.0 * speed.0 * time.delta().as_secs_f32();
    }
}

fn update_shape_transforms(
    mut shapes: Query<(&mut CollisionShape, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut shape, transform) in shapes.iter_mut() {
        shape.set_transform(*transform);
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
    mut score: ResMut<Score>,
    mut ball: Query<
        (&mut Velocity, &mut Transform, &CollisionShape),
        (With<Ball>, Changed<CollisionShape>, Without<Paddle>),
    >,
    paddles: Query<(&Transform, &CollisionShape), (With<Paddle>, Without<Ball>)>,
    walls: Query<(&CollisionShape, &Wall)>,
) {
    let (mut ball_velocity, mut ball_transform, ball_shape) = ball.single_mut();
    for (paddle_transform, paddle_shape) in paddles.iter() {
        if paddle_shape.is_collided_with(ball_shape) {
            ball_velocity.0 =
                (ball_transform.translation - paddle_transform.translation).normalize();
        }
    }
    for (wall_shape, kind) in walls.iter() {
        if wall_shape.is_collided_with(ball_shape) {
            match kind {
                Wall::Horizontal => ball_velocity.0.y *= -1.,
                Wall::Vertical(Side::Left) => {
                    score.right += 1;
                    ball_velocity.0 = Vec3::X;
                    *ball_transform = Transform {
                        rotation: Quat::from_rotation_z(PI / 4.0),
                        ..Default::default()
                    };
                    dbg!(&score);
                }
                Wall::Vertical(Side::Right) => {
                    score.left += 1;
                    ball_velocity.0 = -Vec3::X;
                    *ball_transform = Transform {
                        rotation: Quat::from_rotation_z(PI / 4.0),
                        ..Default::default()
                    };
                    dbg!(&score);
                }
            }
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
