use bevy::prelude::*;

mod board;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    Puzzle,
}

struct MainCamera;
struct UiCamera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(hello.system())
        )
        // .add_plugin(board::BoardPlugin)
        // .add_startup_system(spawn_cameras.system())
        .run();
}

fn hello() {
    print!("Hello");
}

fn spawn_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}