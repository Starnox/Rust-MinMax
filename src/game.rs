use bevy::{prelude::*, app::AppExit, utils::HashMap};
use bevy::math::Vec3Swizzles;
use crate::{GameState, TileMap, MatrixSize, Tile, constants, Coordinates, Bounds2, Board, menu::{get_menu_styles, MenuButtonAction}, despawn_screen};

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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
struct PlayingItem;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayingState::Init)
            .add_state(WhoseTurn::Noone)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(SystemSet::on_update(PlayingState::Playing)
                            .with_system(input_handling)
                            .with_system(render_piece)
                            .with_system(game_button_action))
            .add_system_set(SystemSet::on_exit(PlayingState::Playing)
                            .with_system(despawn_screen::<PlayingItem>));
    }
}


//fn adaptive_tile_size (window: Option<Res<WindowDescriptor>>,
//                       (min, max): (f32, f32),
//                       size)
fn game_setup(mut commands: Commands,
              mut playing_states: ResMut<State<PlayingState>>,
              mut whose_turn: ResMut<State<WhoseTurn>>,
              asset_server: Res<AssetServer>,
              size: Res<MatrixSize>) {

    // Mark that the following player is the one that plays with X
    let _ = playing_states.set(PlayingState::Playing);
    let _ = whose_turn.set(WhoseTurn::XTurn); 
    // Create an empty TileMap and insert the resource
    let tile_map = TileMap::empty((*size).0);

    let mut coord_to_tile = HashMap::with_capacity((*size).0 as usize * (*size).0 as usize);

    let tile_size = constants::LENGTH / (*size).0 as f32;

    let start_x = -(constants::LENGTH / 2.0);
    let start_y = -(constants::LENGTH / 2.0);

    let board_position = Vec3::new(0.,0.,0.);
    let board_size = Vec2::new(constants::LENGTH, constants::LENGTH);


    // Spawn the Back button
    let (_, button_style, button_text_style) = get_menu_styles(asset_server);
    commands.spawn_bundle(ButtonBundle {
        style: button_style.clone(),
        color: constants::NORMAL_BUTTON.into(),
        ..default()
    })
    .insert(Name::new("BackButton"))
    .insert(Transform::from_translation(board_position))
    .insert(GlobalTransform::default())
    .insert(MenuButtonAction::BackToMainMenu)
    .insert(PlayingItem)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            constants::BACK_STRING,
            button_text_style.clone(),
        ));
    });


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
    .insert(PlayingItem)
    .with_children(|parent| {
        for (y, line) in tile_map.0.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                let coordinates: Coordinates = Coordinates {
                    x: y as u16,
                    y: x as u16,
                };
                let entity = parent.spawn_bundle(SpriteBundle {
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
                .insert(coordinates)
                .insert(PlayingItem)
                .id();

                coord_to_tile.insert(coordinates, entity);
            }
        }
    });

    let board = Board {
        tile_map,
        bounds: Bounds2 {
           position: Vec2::new(start_y, start_y),
           size: constants::LENGTH, 
        },
        tile_size,
        coord_to_tile 
    };
    commands.insert_resource(board);
}

fn input_handling(windows: Res<Windows>,
                  mut board: ResMut<Board>,
                  mut whose_turn: ResMut<State<WhoseTurn>>,
                  buttons: Res<Input<MouseButton>>,
                  mut commands: Commands) {
    let window = windows.get_primary().unwrap();
    
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let tile_coordinates = board.mouse_position(window, pos);

            if let Some(coordinates) = tile_coordinates {
                println!("{:?}", coordinates);
                let selected_tile: &mut Tile = 
                    &mut board.tile_map.0[coordinates.x as usize][coordinates.y as usize];

                if *selected_tile == Tile::Empty {
                    match whose_turn.current() {
                        WhoseTurn::XTurn => {
                            *selected_tile = Tile::X;
                            whose_turn.set(WhoseTurn::OTurn).unwrap();
                            spawn_piece(&mut commands,
                                        board.coord_to_tile.get(&coordinates),
                                        &WhoseTurn::XTurn);
                        }
                        WhoseTurn::OTurn => {
                            *selected_tile = Tile::O;
                            whose_turn.set(WhoseTurn::XTurn).unwrap();
                            spawn_piece(&mut commands,
                                        board.coord_to_tile.get(&coordinates),
                                        &WhoseTurn::OTurn);
                        }
                        _ => ()
                    }
                }
                else {
                    println!("Tile already pressed");
                }

                // display the board
                board.tile_map.console_output();
            } 
        }
    }
}


fn game_button_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction),(Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<State<GameState>>,
    mut playing_states: ResMut<State<PlayingState>>
    )
{
    for (interaction, menu_button_action) in interaction_query.iter() {
        if interaction == &Interaction::Clicked {
            if let MenuButtonAction::BackToMainMenu = menu_button_action {
                game_state.set(GameState::Menu).unwrap();
                playing_states.set(PlayingState::Init).unwrap();
            }
        }
    }   
}

fn spawn_piece (commands: &mut Commands,
                entity: Option<&Entity>,
                whose_turn: &WhoseTurn) {

    if let Some(tile) = entity {
        match *whose_turn {
            WhoseTurn::XTurn => {
                commands.entity(*tile).insert(Tile::X);
            },
            WhoseTurn::OTurn => {
                commands.entity(*tile).insert(Tile::O);
            },
            _ => ()
        }
    }
}

fn render_piece (mut commands: Commands,
                 board: Res<Board>,
                 tile_changed: Query<(&Coordinates, &Transform, &Tile), Added<Tile>>) {

    for (_, pos, tile_type) in tile_changed.iter() {
        println!("{:?}", pos);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: match tile_type {
                    Tile::X => Color::RED,
                    Tile::O => Color::BLUE,
                    _ => Color::WHITE
                },
                custom_size: Some(Vec2::splat(board.tile_size * 0.5)),
                ..default()
            },
            transform: Transform::from_xyz(pos.translation.x, -pos.translation.y, 3.0),
            ..default()

            }
        )
        .insert(PlayingItem);
    }

}
