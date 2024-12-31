use crate::defs::{PLAYER_BULLET_SPEED, PLAYER_SPEED, PLAY_RELOAD, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::player::{move_player, shoot_player_bullet, spawn_player};
use crate::stage::init_stage;
use bevy::prelude::*;

mod defs;
mod player;
mod stage;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shooter".into(),
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_linear()),
        )
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::srgb_u8(32, 32, 32)))
        .add_systems(Update, move_player.before(apply_velocity))
        .add_systems(Update, shoot_player_bullet.before(apply_velocity))
        .add_systems(Update, apply_velocity)
        .run();
}

#[derive(Component)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    init_stage(commands, asset_server);
}

fn is_entity_visible(transform: &Transform, image: &Image) -> bool {
    let width = image.size().x;
    let height = image.size().y;

    true
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
