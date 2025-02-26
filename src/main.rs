#![allow(unused, dead_code)]

mod board_plugin;
mod piece_plugin;

#[cfg(debug_assertions)]
use bevy_dylib;
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::oklch(0.269, 0.0, 0.0);

fn main() {
    std::env::set_var("WGPU_BACKEND", "vulkan");

    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
            // board_plugin::BoardPlugin,
            piece_plugin::PiecePlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}