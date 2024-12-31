use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Camera2d, Commands, Res};
use crate::player::spawn_player;

pub fn init_stage(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    init_player(commands, asset_server);
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_player(commands, asset_server, Vec2::default());
}