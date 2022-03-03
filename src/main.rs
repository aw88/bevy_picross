use bevy::prelude::*;

mod board;
mod main_menu;
mod solution;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Puzzle,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Puzzle)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(board::BoardPlugin)
        .add_startup_system(spawn_cameras.system())
        .run();
}

fn spawn_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}