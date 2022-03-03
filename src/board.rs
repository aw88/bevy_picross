use bevy::prelude::*;
use crate::AppState;
use crate::solution::Solution;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<input::CellClick>()
            .init_resource::<input::CellIndex>()
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
            )
            .add_system_set(
                SystemSet::on_update(AppState::Puzzle)
                    .with_system(input::cell_click.system())
                    .with_system(input::index_cells.system())
            );
    }
}

#[derive(Component)]
pub struct Cell;

#[derive(Component, Eq, PartialEq, Hash, Debug)]
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

    pub fn spawn_grid(mut commands: Commands) {
        for grid_line in 0..=GRID_CELL_SIZE {
            commands.spawn_bundle(new_gridline(
                Orientation::Horizontal,
                grid_line,
                GRID_COLOR,
            ));

            commands.spawn_bundle(new_gridline(
                Orientation::Vertical,
                grid_line,
                GRID_COLOR,
            ));
        }
    }

    fn new_gridline(
        orientation: Orientation,
        index: u8,
        grid_color: Color,
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
            sprite: Sprite {
                custom_size: Some(size),
                color: grid_color,
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..Default::default()
        }
    }

    pub fn spawn_cells(mut commands: Commands) {
        let cell_handle = CELL_COLOR;
        let cell_handle_alt = CELL_COLOR_ALT;

        for row in 1..=GRID_CELL_SIZE {
            for column in 1..=GRID_CELL_SIZE {
                let cell_color = match get_cell_color(row, column) {
                    true => cell_handle,
                    false => cell_handle_alt,
                };

                commands.spawn_bundle(CellBundle::new(row, column, cell_color));
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
        fn new(column: u8, row: u8, cell_color: Color) -> Self {
            let x = GRID_LEFT_EDGE + CELL_SIZE * column as f32 - 0.5 * CELL_SIZE;
            let y = GRID_BOTTOM_EDGE + CELL_SIZE * row as f32 - 0.5 * CELL_SIZE;

            CellBundle {
                cell: Cell,
                coordinates: Coordinates {
                    x: column,
                    y: row,
                },
                cell_fill: SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        color: cell_color,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..Default::default()
                },
            }
        }
    }
}

mod input {
    use bevy::utils::HashMap;
    use crate::board::config::{CELL_SIZE, GRID_BOTTOM_EDGE, GRID_LEFT_EDGE};
    use super::*;
    use crate::MainCamera;

    #[derive(Default)]
    pub struct CellIndex {
        pub cell_map: HashMap<Coordinates, Entity>,
    }

    pub struct CellClick {
        pub selected_cell: Option<Entity>,
    }

    pub fn cell_click(
        camera_query: Query<&Transform, With<MainCamera>>,
        cell_query: Query<(&Cell, &Coordinates)>,
        mouse_button_input: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        cell_index: Res<CellIndex>,
        mut cell_click_events: EventWriter<CellClick>,
    ) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let window = windows.get_primary().expect("Primary window not found.");
            let mut cursor_position = window
                .cursor_position()
                .expect("Cursor position not found.");

            let camera_transform = camera_query.get_single().expect("MainCamera not found.");
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);

            cursor_position -= 0.5 * window_size;

            let world_quaternion = camera_transform.compute_matrix() * cursor_position.extend(0.0).extend(0.0);

            let cursor_position_world = Vec2::new(world_quaternion.x, world_quaternion.y);
            let tile_coord = (cursor_position_world - Vec2::new(GRID_LEFT_EDGE, GRID_BOTTOM_EDGE)) / CELL_SIZE;
            let coordinates = Coordinates {
                x: tile_coord.x as u8,
                y: tile_coord.y as u8,
            };

            let selected_cell = cell_index.cell_map.get(&coordinates)
                .map(|e| *e);

            if selected_cell.is_some() {
                let (_cell, coordinates) = cell_query.get(selected_cell.unwrap()).unwrap();
                
                println!("Clicked cell:  {:?}", selected_cell.unwrap());
                println!("  coordinates: {:?}", coordinates);
            }

            cell_click_events.send(CellClick {
                selected_cell,
            });
        }
    }

    pub fn index_cells(
        query: Query<(Entity, &Transform), (With<Cell>, Changed<Transform>)>,
        mut cell_index: ResMut<CellIndex>,
    ) {
        for (entity, transform) in query.iter() {
            let center: Vec2 = transform.translation.truncate();
            let tile_coord: Vec2 = (center - Vec2::new(GRID_LEFT_EDGE, GRID_BOTTOM_EDGE)) / CELL_SIZE;
            let coordinates = Coordinates {
                x: tile_coord.x as u8,
                y: tile_coord.y as u8,
            };

            println!("Indexing tile: {:?}", coordinates);
            cell_index.cell_map.insert(coordinates, entity);
        }
    }
}