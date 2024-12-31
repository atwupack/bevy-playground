use crate::defs::{PLAYER_BULLET_SPEED, PLAYER_SPEED, PLAY_RELOAD};
use crate::Velocity;
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, KeyCode, Query, Res, Sprite, Time, Timer, TimerMode, Transform, With,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
pub struct PlayerShootTimer(Timer);

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, position: Vec2) {
    let mut timer = Timer::from_seconds(PLAY_RELOAD, TimerMode::Once);
    timer.pause();
    commands.spawn((
        Player,
        PlayerShootTimer(timer),
        Velocity(Vec2::default()),
        Sprite::from_image(asset_server.load("gfx/player.png")),
        Transform::from_xyz(position.x, position.y, 0.0),
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut player_velocity = query.single_mut();

    player_velocity.0.x = 0.0;
    player_velocity.0.y = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player_velocity.0.y = PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        player_velocity.0.y = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player_velocity.0.x = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player_velocity.0.x = PLAYER_SPEED;
    }
}

pub fn shoot_player_bullet(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut PlayerShootTimer), With<Player>>,
) {
    let (player_transform, mut shoot_timer) = query.single_mut();
    shoot_timer.0.tick(time.delta());

    if keyboard_input.pressed(KeyCode::ShiftLeft)
        && (shoot_timer.0.paused() || shoot_timer.0.finished())
    {
        shoot_timer.0.reset();
        shoot_timer.0.unpause();
        commands.spawn((
            PlayerBullet,
            Velocity(Vec2::new(PLAYER_BULLET_SPEED, 0.)),
            Sprite::from_image(asset_server.load("gfx/playerBullet.png")),
            player_transform.clone(),
        ));
    };
}
