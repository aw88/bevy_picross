use bevy::prelude::*;
use crate::AppState;
use crate::solution::Solution;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Solution {
                size: (10, 10),
                tiles: vec![
                    vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1,],
                    vec![1, 1, 0, 1, 1, 0, 0, 0, 1, 1,],
                    vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 0,],
                    vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 1,],
                    vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0,],
                    vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 1,],
                    vec![0, 1, 0, 1, 1, 0, 0, 0, 1, 1,],
                    vec![0, 1, 1, 0, 0, 0, 1, 0, 0, 0,],
                    vec![0, 1, 1, 0, 0, 1, 1, 1, 0, 0,],
                    vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 1,],
                ]
            })
            .add_system_set(
                SystemSet::on_enter(AppState::Puzzle)
                    .with_system(setup::spawn_grid.system())
                    .with_system(setup::spawn_cells.system())
            );
    }
}

pub struct Cell;
pub struct Coordinates {
    x: u8,
    y: u8,
}

mod config {
    use super::*;

    pub const CELL_COLOR: Color = Color::rgb(0.95, 0.95, 0.95);
    pub const CELL_COLOR_ALT: Color = Color::rgb(0.75, 0.75, 0.75);
    pub const GRID_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

    pub const CELL_SIZE: f32 = 32.0;
    pub const GRID_CELL_SIZE: u8 = 10;
    pub const GRID_SIZE: f32 = GRID_CELL_SIZE as f32 * CELL_SIZE;
    pub const MINOR_LINE_THICKNESS: f32 = 2.0;
    pub const MAJOR_LINE_THICKNESS: f32 = 4.0;

    pub const GRID_CENTER_X: f32 = 0.0;
    pub const GRID_LEFT_EDGE: f32 = GRID_CENTER_X - 0.5 * GRID_SIZE;
    pub const GRID_CENTER_Y: f32 = 0.0;
    pub const GRID_BOTTOM_EDGE: f32 = GRID_CENTER_Y - 0.5 * GRID_SIZE;
}

mod setup {
    use super::*;
    use super::config::*;

    enum Orientation {
        Horizontal,
        Vertical,
    }

    pub fn spawn_grid(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
        let grid_handle = materials.add(GRID_COLOR.into());

        for grid_line in 0..=GRID_CELL_SIZE {
            commands.spawn_bundle(new_gridline(
                Orientation::Horizontal,
                grid_line,
                grid_handle.clone(),
            ));

            commands.spawn_bundle(new_gridline(
                Orientation::Vertical,
                grid_line,
                grid_handle.clone(),
            ));
        }
    }

    fn new_gridline(
        orientation: Orientation,
        index: u8,
        grid_handle: Handle<ColorMaterial>,
    ) -> SpriteBundle {
        let thickness = if (index % 5) == 0 {
            MAJOR_LINE_THICKNESS
        } else {
            MINOR_LINE_THICKNESS
        };

        let length = GRID_SIZE + thickness;

        let size = match orientation {
            Orientation::Horizontal => Vec2::new(length, thickness),
            Orientation::Vertical => Vec2::new(thickness, length),
        };

        let offset = index as f32 * CELL_SIZE;

        let (x, y) = match orientation {
            Orientation::Horizontal => (GRID_LEFT_EDGE + 0.5 * GRID_SIZE, GRID_BOTTOM_EDGE + offset),
            Orientation::Vertical => (GRID_LEFT_EDGE + offset, GRID_BOTTOM_EDGE + 0.5 * GRID_SIZE),
        };

        SpriteBundle {
            sprite: Sprite::new(size),
            transform: Transform::from_xyz(x, y, 1.0),
            material: grid_handle,
            ..Default::default()
        }
    }

    pub fn spawn_cells(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
        let cell_handle = materials.add(CELL_COLOR.into());
        let cell_handle_alt = materials.add(CELL_COLOR_ALT.into());

        for row in 1..=GRID_CELL_SIZE {
            for column in 1..=GRID_CELL_SIZE {
                let handle = match get_cell_color(row, column) {
                    true => cell_handle.clone(),
                    false => cell_handle_alt.clone(),
                };

                commands.spawn_bundle(CellBundle::new(row, column, handle));
            }
        }
    }

    // Simple checkerboard pattern between 5x5 blocks
    fn get_cell_color(row: u8, column: u8) -> bool {
        (((row - 1) / 5) ^ ((column - 1) / 5)) & 1 == 1
    }

    #[derive(Bundle)]
    struct CellBundle {
        cell: Cell,
        coordinates: Coordinates,
        #[bundle]
        cell_fill: SpriteBundle,
    }

    impl CellBundle {
        fn new(column: u8, row: u8, cell_handle: Handle<ColorMaterial>) -> Self {
            let x = GRID_LEFT_EDGE + CELL_SIZE * column as f32 - 0.5 * CELL_SIZE;
            let y = GRID_BOTTOM_EDGE + CELL_SIZE * row as f32 - 0.5 * CELL_SIZE;

            CellBundle {
                cell: Cell,
                coordinates: Coordinates {
                    x: column,
                    y: row,
                },
                cell_fill: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    material: cell_handle,
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..Default::default()
                },
            }
        }
    }
}

mod input {
    use super::*;
    use crate::MainCamera;

    pub struct CellClick {
        pub selected_cell: Option<Entity>,
    }

    pub fn cell_click(
        camera_query: Query<&Transform, With<MainCamera>>,
        mouse_button_input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut cell_click_events: EventWriter<CellClick>,
    ) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let window = windows.get_primary().expect("Primary window not found.");
            let mut cursor_position = window
                .cursor_position()
                .expect("Cursor position not found.");

            let camera_transform = camera_query.single().expect("MainCamera not found.");
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);

            cursor_position -= 0.5 * window_size;

            let world_quaternion = camera_transform.compute_matrix() * cursor_position.extend(0.0).extend(0.0);

            let cursor_position_world = Vec2::new(world_quaternion.x, world_quaternion.y);

            let selected_cell = None; // TODO: Find the right cell

            cell_click_events.send(CellClick {
                selected_cell,
            });
        }
    }
}