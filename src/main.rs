use std::f32::consts::PI;

use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*, transform::TransformSystem};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use impacted::CollisionShape;

mod components;
use components::*;

const TIMESTEP: f32 = 1. / 60.;

#[derive(Debug)]
struct Score {
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    InGame,
}
impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.left, self.right))
    }
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(Score { left: 0, right: 0 })
        .add_state(GameState::Menu)
        .add_system(score_ui)
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_ui))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP as f64).chain(
                    |In(input): In<ShouldRun>, state: Res<State<GameState>>| {
                        if state.current() == &GameState::InGame {
                            input
                        } else {
                            ShouldRun::No
                        }
                    },
                ))
                .with_system(update_shape_transforms)
                .with_system(check_collisions.after(update_shape_transforms))
                .with_system(apply_velocity.before(check_collisions))
                .with_system(move_paddle.before(check_collisions))
                .with_system(rotate_paddle.before(check_collisions)),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity, &Speed)>) {
    for (mut transform, velocity, speed) in query.iter_mut() {
        transform.translation += velocity.0 * speed.0 * TIMESTEP
    }
}

fn score_ui(mut ctx: ResMut<EguiContext>, score: Res<Score>) {
    let default_fonts = egui::FontDefinitions::default();
    let mut fonts = default_fonts.clone();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/bit5x3.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    ctx.ctx_mut().set_fonts(fonts);
    egui::Area::new("Score")
        .anchor(egui::Align2::CENTER_TOP, [0.0, 20.0])
        .show(ctx.ctx_mut(), |ui| {
            ui.label(
                egui::RichText::new(format!("{}", score.into_inner()))
                    .strong()
                    .size(100.),
            )
        });
}

fn menu_ui(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut score: ResMut<Score>,
    mut ctx: ResMut<EguiContext>,
) {
    egui::Area::new("Test")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx.ctx_mut(), |ui| {
            ui.label("Press space to start");
        });
    if input.pressed(KeyCode::Space) {
        state.set(GameState::InGame).unwrap();
        score.left = 0;
        score.right = 0;
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
    mut state: ResMut<State<GameState>>,
    mut score: ResMut<Score>,
    mut ball: Query<
        (&mut Velocity, &mut Transform, &CollisionShape),
        (With<Ball>, Without<Paddle>),
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
                Wall::Horizontal(HSide::Bottom) => {
                    if ball_velocity.0.y > 0. {
                        ball_velocity.0.y *= -1.
                    }
                }
                Wall::Horizontal(HSide::Top) => {
                    if ball_velocity.0.y < 0. {
                        ball_velocity.0.y *= -1.
                    }
                }
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
    if score.right >= 5 || score.left >= 5 {
        state.set(GameState::Menu).unwrap();
    }
}

fn rotate_paddle(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &InputConfig), With<Paddle>>,
) {
    for (mut transform, keys) in query.iter_mut() {
        if input.pressed(keys.rot_1) {
            transform.rotation *= Quat::from_rotation_z(TIMESTEP * 2.);
        }
        if input.pressed(keys.rot_2) {
            transform.rotation *= Quat::from_rotation_z(TIMESTEP * -2.);
        }
    }
}
