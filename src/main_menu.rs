use bevy::prelude::*;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup::create_menu.system())
            )
            .add_system_set(SystemSet::on_update(AppState::MainMenu)
                .with_system(input::handle_buttons.system())
            );
    }
}

#[derive(Component)]
struct PlayButton;

pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    clicked: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        ButtonMaterials {
            normal: materials.add(config::BUTTON_COLOR.into()),
            hovered: materials.add(config::BUTTON_HOVERED_COLOR.into()),
            clicked: materials.add(config::BUTTON_CLICKED_COLOR.into()),
        }
    }
}

mod config {
    use super::*;

    pub const BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const BUTTON_HOVERED_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const BUTTON_CLICKED_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

    pub const BUTTON_TEXT_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
}

mod setup {
    use super::*;
    use super::config::*;

    pub fn create_menu(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: BUTTON_COLOR.into(),
                ..Default::default()
            })
            .insert(PlayButton)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Play!".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: BUTTON_TEXT_COLOR,
                                },
                            }],
                            alignment: Default::default()
                        },
                        ..Default::default()
                    });
            });
    }
}

mod input {
    use super::*;

    type ButtonInteraction<'a> = (
        Entity,
        &'a Interaction,
        &'a mut Handle<ColorMaterial>,
    );

    pub fn handle_buttons(
        mut commands: Commands,
        button_materials: Res<ButtonMaterials>,
        mut interaction_query: Query<
            ButtonInteraction,
            (Changed<Interaction>, With<Button>),
        >,
        mut app_state: ResMut<State<AppState>>,
    ) {
        for (button, interaction, mut material) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => {
                    *material = button_materials.clicked.clone();
                    commands.entity(button).despawn_recursive();
                    app_state.set(AppState::Puzzle).unwrap();
                },
                Interaction::Hovered => {
                    *material = button_materials.hovered.clone();
                },
                Interaction::None => {
                    *material = button_materials.normal.clone();
                }
            }
        }
    }
}