use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::defs::{PLAY_RELOAD, PLAYER_BULLET_SPEED, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};

mod defs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title : "Shooter".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_linear())
        )
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::rgb_u8(96, 128, 255)))
        .add_system(move_player.before(apply_velocity))
        .add_system(shoot_player_bullet.before(apply_velocity))
        .add_system(apply_velocity)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerBullet;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct PlayerShootTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let mut timer = Timer::from_seconds(PLAY_RELOAD, TimerMode::Once);
    timer.pause();
    commands.spawn((
        Player,
        PlayerShootTimer(timer),
        Velocity(Vec2::default()),
        SpriteBundle {
            texture: asset_server.load("gfx/player.png"),
            transform: Transform::from_xyz(-540., 260., 1.),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        })
    );
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = query.single_mut();

    player_velocity.0.x = 0.0;
    player_velocity.0.y = 0.0;
    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.0.y = PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.0.y = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.0.x = -PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.0.x = PLAYER_SPEED;
    }
}

fn shoot_player_bullet(time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Transform, &mut PlayerShootTimer), With<Player>>) {
    let (player_transform, mut shoot_timer) = query.single_mut();
    shoot_timer.0.tick(time.delta());

    if keyboard_input.pressed(KeyCode::LShift) && (shoot_timer.0.paused() || shoot_timer.0.finished()) {
        shoot_timer.0.reset();
        shoot_timer.0.unpause();
        commands.spawn((
            PlayerBullet,
            Velocity(Vec2::new(PLAYER_BULLET_SPEED, 0.)),
            SpriteBundle {
                texture: asset_server.load("gfx/playerBullet.png"),
                transform: player_transform.clone(),
                sprite: Sprite {
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            })
        );
    };
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}