use bevy::{prelude::*, app::AppExit};
use bevy::math::Vec3Swizzles;
use crate::{GameState, TileMap, MatrixSize, Tile, constants, Coordinates, Bounds2, Board};

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PlayingState {
    Init,
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum WhoseTurn {
    Noone,
    XTurn,
    OTurn,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayingState::Init)
            .add_state(WhoseTurn::Noone)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(SystemSet::on_update(PlayingState::Playing).with_system(input_handling));
    }
}


//fn adaptive_tile_size (window: Option<Res<WindowDescriptor>>,
//                       (min, max): (f32, f32),
//                       size)
fn game_setup(mut commands: Commands,
              mut playing_states: ResMut<State<PlayingState>>,
              mut whose_turn: ResMut<State<WhoseTurn>>,
              size: Res<MatrixSize>) {

    // Mark that the following player is the one that plays with X
    let _ = playing_states.set(PlayingState::Playing);
    let _ = whose_turn.set(WhoseTurn::XTurn); 
    // Create an empty TileMap and insert the resource
    let tile_map = TileMap::empty((*size).0);

    let tile_size = constants::LENGTH / (*size).0 as f32;

    let start_x = -(constants::LENGTH / 2.0);
    let start_y = -(constants::LENGTH / 2.0);

    let board_position = Vec3::new(0.,0.,0.);
    let board_size = Vec2::new(constants::LENGTH, constants::LENGTH);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(board_size),
            ..default()
        },
        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2.0, 0.),
        ..default()
    })
    .insert(Name::new("Board"))
    .insert(Transform::from_translation(board_position))
    .insert(GlobalTransform::default())
    .with_children(|parent| {
        for (y, line) in tile_map.0.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(tile_size - constants::TILE_PADDING)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        start_x + (x as f32 * tile_size) + (tile_size / 2.0),
                        start_y + (y as f32 * tile_size) + (tile_size / 2.0),
                        1.0
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                .insert(Coordinates {
                    x: x as u16,
                    y: y as u16,
                });
            }
        }
    });

    let board = Board {
        tile_map,
        bounds: Bounds2 {
           position: Vec2::new(start_y, start_y),
           size: constants::LENGTH, 
        },
        tile_size
    };
    commands.insert_resource(board);
}

fn input_handling(windows: Res<Windows>,
                  board: Res<Board>,
                  buttons: Res<Input<MouseButton>>) {
    let window = windows.get_primary().unwrap();
    
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let tile_coordinates = board.mouse_position(window, pos);

            if let Some(coordinates) = tile_coordinates {
                println!("{:?}", coordinates);
            } 
        }
    }
}
