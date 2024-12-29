use bevy::prelude::*;
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
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::srgb_u8(32, 32, 32)))
        .add_systems(Update, move_player.before(apply_velocity))
        .add_systems(Update, shoot_player_bullet.before(apply_velocity))
        .add_systems(Update, apply_velocity)
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
    commands.spawn(Camera2d::default());
    let mut timer = Timer::from_seconds(PLAY_RELOAD, TimerMode::Once);
    timer.pause();
    commands.spawn((
        Player,
        PlayerShootTimer(timer),
        Velocity(Vec2::default()),
        Sprite::from_image(asset_server.load("gfx/player.png")),

        )
    );
}

fn is_entity_visible(transform: &Transform, image: &Image) -> bool {
    let width = image.size().x;
    let height = image.size().y;

    true
}

fn move_player(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
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

fn shoot_player_bullet(time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>, keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Transform, &mut PlayerShootTimer), With<Player>>) {
    let (player_transform, mut shoot_timer) = query.single_mut();
    shoot_timer.0.tick(time.delta());

    if keyboard_input.pressed(KeyCode::ShiftLeft) && (shoot_timer.0.paused() || shoot_timer.0.finished()) {
        shoot_timer.0.reset();
        shoot_timer.0.unpause();
        commands.spawn((
            PlayerBullet,
            Velocity(Vec2::new(PLAYER_BULLET_SPEED, 0.)),
            Sprite::from_image(asset_server.load("gfx/playerBullet.png"))
        ));
    };
}



fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let mut count = 0;
    for (mut transform, velocity) in &mut query {
        count += 1;
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
    info!("Found {} entities.", count)
}

