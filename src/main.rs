use bevy::time::FixedTimestep;
use bevy::prelude::*;

use crate::menu::MenuPlugin;

mod constants;
mod menu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct MatrixSize (pub u32);

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AiDepth (pub u32);

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume (pub u32);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MatrixSize(constants::DEFAULT_BOARD_SIZE))
        .insert_resource(AiDepth(constants::DEFAULT_AI_DEPTH))
        .insert_resource(Volume(constants::DEFAULT_VOLUME))
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_plugin(MenuPlugin)
        .run();
}

fn setup(mut commands: Commands) {
   commands.spawn_bundle(Camera2dBundle::default()); 
}

fn cursor_position(
    windows: Res<Windows>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        println!("{}", position.to_string());
    } else {
        // cursor is not inside the window
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

