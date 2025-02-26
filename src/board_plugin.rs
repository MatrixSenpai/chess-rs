use bevy::prelude::*;

const WHITE_SQUARE_COLOR: Color = Color::oklch(0.929, 0.013, 255.508);
const BLACK_SQUARE_COLOR: Color = Color::oklch(0.13, 0.028, 261.692);

#[derive(Component, Reflect)]
pub struct SquareCoord(u32, u32);

#[derive(Bundle, Reflect)]
pub struct BoardSquare {
    coord: SquareCoord,
    transform: Transform,
    sprite: Sprite,
}
impl BoardSquare {
    pub fn new(x: u32, y: u32) -> Self {
        let x_location = (x as f32 - 5.0) * 0.1 + 0.05;
        let y_location = (y as f32 - 5.0) * 0.1 + 0.05;

        Self {
            coord: SquareCoord(x, y),
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 1.0),
                translation: Vec3::new(x_location, y_location, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: if (x + y) % 2 == 0 { BLACK_SQUARE_COLOR } else { WHITE_SQUARE_COLOR },
                ..default()
            },
        }
    }
}

#[derive(Bundle, Reflect)]
struct Board {
    sprite: Sprite,
    transform: Transform,
}
impl Board {
    pub fn new() -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(700.0, 700.0, 1.0),
                ..default()
            },
        }
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SquareCoord>()
            .register_type::<BoardSquare>()
            .register_type::<Board>()
            .add_systems(PostStartup, setup_board);
    }
}

fn setup_board(mut commands: Commands) {
    commands.spawn(Board::new())
        .with_children(|board| {
            for row in 0..10 {
                for col in 0..10 {
                    board.spawn(BoardSquare::new(row, col));
                }
            }
        });
}