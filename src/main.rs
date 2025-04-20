use bevy::prelude::*;

const VELOCITY_CHANGE_INTERVAL: f32 = 0.1;
const START_MAX_RANDOM_VELOCITY: f32 = 300.0;
const MAX_RANDOM_VELOCITY_INCREASE_STEP: f32 = 0.5;
const BIAS_ACCELERATION: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (walker_movement, velocity_random_walk, player_bias))
        .run();
}

#[derive(Component)]
struct Walker{
    pub velocity: Vec2,
    pub max_velocity: f32,
    pub velocity_change_timer: Timer,
    pub bias: Vec2,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite{
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Walker{
            velocity: Vec2::ZERO,
            max_velocity: START_MAX_RANDOM_VELOCITY,
            velocity_change_timer: Timer::from_seconds(VELOCITY_CHANGE_INTERVAL, TimerMode::Repeating),
            bias: Vec2::ZERO,
        }
    ));
}

fn walker_movement(
    time: Res<Time>,
    mut walker_query: Query<(&mut Walker, &mut Transform)>
) {
    for (walker, mut transform) in &mut walker_query {
        transform.translation.x += (walker.velocity.x + walker.bias.x) * time.delta_secs();
        transform.translation.y += (walker.velocity.y + walker.bias.y) * time.delta_secs();
    }
}

fn velocity_random_walk(
    time: Res<Time>,
    mut walker_query: Query<&mut Walker>
) {
    for mut walker in &mut walker_query {
        if walker.velocity_change_timer.tick(time.delta()).just_finished() {
            walker.max_velocity += MAX_RANDOM_VELOCITY_INCREASE_STEP;

            let random_x = rand::random::<f32>() * walker.max_velocity;
            let random_y = rand::random::<f32>() * walker.max_velocity;
            
            let x_direction = if rand::random::<bool>() { 1.0 } else { -1.0 };
            let y_direction = if rand::random::<bool>() { 1.0 } else { -1.0 };
            
            walker.velocity = Vec2::new(random_x * x_direction, random_y * y_direction);
        }
    }
}

fn player_bias(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut walker_query: Query<&mut Walker>
) {
    for mut walker in &mut walker_query {
        if keyboard_input.pressed(KeyCode::KeyW) {
            walker.bias.y += BIAS_ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            walker.bias.x -= BIAS_ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            walker.bias.y -= BIAS_ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            walker.bias.x += BIAS_ACCELERATION;
        }
    }
}
