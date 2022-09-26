use bevy::{prelude::*, app::AppExit};

use crate::{constants::{self, PRESSED_BUTTON, HOVERED_PRESS_BUTTON, NORMAL_BUTTON, HOVERED_BUTTON, TEXT_COLOR, GAME_STRING_FONT_SIZE},
GameState, MatrixSize, AiDepth, despawn_screen};

pub struct MenuPlugin;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings screen
#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct OnMatrixSizeMenuScreen;

#[derive(Component)]
struct OnAiDepthMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click 
#[derive(Component)]
pub enum MenuButtonAction {
    PlayAi,
    PlayPlayers,
    Settings,
    SettingsMatrixSize,
    SettingsAiDepth,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    Settings,
    SettingsMatrixSize,
    SettingsAiDepth,
    Disabled,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
            .add_system_set(SystemSet::on_exit(MenuState::Main)
                            .with_system(despawn_screen::<OnMainMenuScreen>))

            .add_system_set(SystemSet::on_enter(MenuState::Settings)
                            .with_system(settings_menu_setup))
            .add_system_set(SystemSet::on_exit(MenuState::Settings)
                            .with_system(despawn_screen::<OnSettingsMenuScreen>))

            .add_system_set(SystemSet::on_enter(MenuState::SettingsMatrixSize)
                            .with_system(settings_menu_matrix_size)) 
            .add_system_set(SystemSet::on_update(MenuState::SettingsMatrixSize)
                            .with_system(setting_button::<MatrixSize>))
            .add_system_set(SystemSet::on_exit(MenuState::SettingsMatrixSize)
                            .with_system(despawn_screen::<OnMatrixSizeMenuScreen>))


            .add_system_set(SystemSet::on_enter(MenuState::SettingsAiDepth)
                            .with_system(settings_menu_ai_depth))
            .add_system_set(SystemSet::on_update(MenuState::SettingsAiDepth)
                            .with_system(setting_button::<AiDepth>))
            .add_system_set(SystemSet::on_exit(MenuState::SettingsAiDepth)
                            .with_system(despawn_screen::<OnAiDepthMenuScreen>))

            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system)
            );
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

// Get all the entities in the world that have the Interaction, Color and
// SelectedOption component and then filter them by the components that
// recently have been modified and also have a button component
fn button_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>), >,)
{
    // Go through all of the queried entities and dereference each of the 
    // component with the specified R/W "permissions"
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESS_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a settings is selected,
// and marks the button as the one currently selected
// This is where the global resource is changed 
// In interaction query the current button that is clicked is retrieved
// and in selected_query the previous one. The previous one is reverted to 
// default settings and the new is added the selectedOption component.
// Afterwards the global resource is modifed;
fn setting_button<T: Component + PartialEq + Copy> (
    interaction_query: Query<(&Interaction, &T, Entity),
                            (Changed<Interaction>, With<Button>)>,
    mut selected_querry: Query<(Entity, &mut UiColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
    ) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Clicked && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_querry.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }

}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction),
    (Changed<Interaction>, With<Button>)
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
              )
{
    for (interaction, menu_button_action) in interaction_query.iter() {
        if interaction == &Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::PlayAi | MenuButtonAction::PlayPlayers => {
                    game_state.set(GameState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                },
                MenuButtonAction::Settings => 
                    menu_state.set(MenuState::Settings).unwrap(),

                MenuButtonAction::SettingsMatrixSize =>  
                    menu_state.set(MenuState::SettingsMatrixSize).unwrap(),

                MenuButtonAction::SettingsAiDepth =>  
                    menu_state.set(MenuState::SettingsAiDepth).unwrap(),
                    
                MenuButtonAction::BackToMainMenu =>  
                    menu_state.set(MenuState::Main).unwrap(),

                MenuButtonAction::BackToSettings =>  
                    menu_state.set(MenuState::Settings).unwrap(),

                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
    
}

pub fn get_menu_styles(asset_server: Res<AssetServer>) -> (Handle<Font>, Style, TextStyle) {
    let font = asset_server.load(constants::FONT_LOCATION);
    let button_style = Style {
        size: Size::new(Val::Px(constants::BUTTON_WIDTH), Val::Px(constants::BUTTON_HEIGHT)),
        margin: UiRect::all(Val::Px(constants::BUTTON_MARGIN)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: constants::BUTTON_FONT_SIZE,
        color: TEXT_COLOR, 
    };
    (font, button_style, button_text_style)
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (font, button_style, button_text_style) = get_menu_styles(asset_server);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(constants::LENGTH), Val::Px(constants::LENGTH)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::GRAY.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
            .with_children(|parent| {
                // Display the Game Name
                parent.spawn_bundle(
                    TextBundle::from_section(
                        constants::GAME_STRING,
                        TextStyle {
                            font: font.clone(),
                            font_size: GAME_STRING_FONT_SIZE,
                            color: TEXT_COLOR,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(constants::GAME_STRING_MARGIN)),
                        ..default()
                    }),
                );

                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::PlayAi)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            constants::PLAY_AI_STRING,
                            button_text_style.clone(),
                        ));
                    });
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::PlayPlayers)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            constants::PLAY_AGAINST_PLAYER_STRING,
                            button_text_style.clone(),
                        ));
                    });
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::Settings)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            constants::SETTINGS_STRING,
                            button_text_style.clone(),
                        ));
                    });
                parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::Quit)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            constants::QUIT_STRING,
                            button_text_style.clone(),
                        ));
                    });
            });
}

fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (_, button_style, button_text_style) = get_menu_styles(asset_server);
    commands.spawn_bundle( NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            size: Size::new(Val::Px(constants::LENGTH), Val::Px(constants::LENGTH)),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    })
    .insert(OnSettingsMenuScreen)
    .with_children(|parent| {
        for (action, text) in [
            (MenuButtonAction::SettingsAiDepth, constants::AI_DEPTH_SETTING_STRING),
            (MenuButtonAction::SettingsMatrixSize, constants::MATRIX_SIZE_SETTING_STRING),
            (MenuButtonAction::BackToMainMenu, constants::BACK_STRING),
        ] {
            parent
                .spawn_bundle(
                    ButtonBundle{
                        style: button_style.clone(),
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                .insert(action)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        text,
                        button_text_style.clone(),
                    ));
                });
        };
    });
    
}
fn settings_menu_matrix_size(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    matrix_size: Res<MatrixSize>
    ) {
    let (_, button_style, button_text_style) = get_menu_styles(asset_server);

    commands.spawn_bundle( NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    })
    .insert(OnMatrixSizeMenuScreen)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::GRAY.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                        "Matrix Size",
                        button_text_style.clone(),
                ));
                for current_size in 3..9 {
                    let mut entity = parent.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            ..button_style.clone()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    });
                    entity.insert(MatrixSize(current_size));
                    entity.with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle::from_section(
                                    current_size.to_string(),
                                    button_text_style.clone(),
                            ));
                    });
                    if *matrix_size == MatrixSize(current_size) {
                        entity.insert(SelectedOption);
                    }
                }
            });
            parent.spawn_bundle(ButtonBundle {
                style: button_style,
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(MenuButtonAction::BackToSettings)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                        constants::BACK_STRING,
                        button_text_style,
                ));
            });

        });

}

fn settings_menu_ai_depth(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ai_depth: Res<AiDepth>
    ) {
    let (_, button_style, button_text_style) = get_menu_styles(asset_server);

    commands.spawn_bundle( NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    })
    .insert(OnAiDepthMenuScreen)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::GRAY.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                        "AI Depth",
                        button_text_style.clone(),
                ));
                for current_depth in 3..9 {
                    let mut entity = parent.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            ..button_style.clone()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..default()
                    });
                    entity.insert(AiDepth(current_depth));
                    entity.with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle::from_section(
                                    current_depth.to_string(),
                                    button_text_style.clone(),
                            ));
                    });
                    if *ai_depth == AiDepth(current_depth) {
                        entity.insert(SelectedOption);
                    }
                }
            });
            parent.spawn_bundle(ButtonBundle {
                style: button_style,
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(MenuButtonAction::BackToSettings)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                        constants::BACK_STRING,
                        button_text_style,
                ));
            });

        });
}

