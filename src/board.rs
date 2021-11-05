use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup::spawn_grid.system());
    }
}

mod config {
    use super::*;

    pub const GRID_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

    pub const CELL_SIZE: f32 = 32.0;
    pub const GRID_CELL_SIZE: u8 = 15;
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
}
