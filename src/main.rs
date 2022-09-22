use serde::{Deserialize, Serialize};
use bevy::{prelude::*, utils::HashMap};

use crate::{menu::MenuPlugin, game::GamePlugin};

mod constants;
mod menu;
mod game;

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
pub enum Tile {
    Empty,
    X,
    O,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone)]
pub struct TileMap (Vec<Vec<Tile>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive {min: f32, max: f32},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    // Centered board
    Centered {offset: Vec3},
    // Custom position
    Custom(Vec3),
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: constants::MIN_TILE_SIZE,
            max: constants:: MAX_TILE_SIZE,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered { offset: Default::default() }
    }
}

impl TileMap {
    // Create an empty tile map
    pub fn empty(size: u32) -> Self {
        let map = (0..size).into_iter().
                    map(|_| (0..size).into_iter().map(|_| Tile::Empty).collect())
                 .collect();
        TileMap(map)
    }

    pub fn console_output(&self) {
        for line in self.0.iter() {
            for element in line.iter() {
                print!("{:?} ", element);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2, 
    pub size: f32,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x 
        && coords.x <= self.position.x + self.size
        && coords.y >= self.position.y
        && coords.y <= self.position.y + self.size
    }
}

#[derive(Debug)]
struct Board {
    tile_map: TileMap,
    tile_size: f32,
    bounds: Bounds2,
    coord_to_tile: HashMap<Coordinates, Entity>,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let mut position = position - window_size / 2.;

        position.y = -position.y;


        if !self.bounds.in_bounds(position) {
            return None;
        }

        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.y / self.tile_size ) as u16,
            y: (coordinates.x / self.tile_size ) as u16,
        })
    }
    pub fn get_tile(&self, coordinates: &Coordinates) -> Option<&Entity> {
        return self.coord_to_tile.get(coordinates);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MatrixSize(constants::DEFAULT_BOARD_SIZE))
        .insert_resource(AiDepth(constants::DEFAULT_AI_DEPTH))
        .add_startup_system(setup)
        //.add_system(cursor_position)
        .add_state(GameState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
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

    let window_size = Vec2::new(window.width(), window.height());

    if let Some(position) = window.cursor_position() {
        let mut position = position - window_size / 2.;
        position.y = -position.y;
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

