use bevy::prelude::*;

const WHITE_SQUARE_COLOR: Color = Color::oklch(0.929, 0.013, 255.508);
const BLACK_SQUARE_COLOR: Color = Color::oklch(0.269, 0.0, 0.0);

#[derive(Component, Reflect)]
pub struct SquareCoord(u32, u32);
impl PartialEq<(usize, usize)> for SquareCoord {
    fn eq(&self, other: &(usize, usize)) -> bool {
        (self.0 as usize).eq(&other.0) && (self.1 as usize).eq(&other.1)
    }
}

#[derive(Component, Reflect)]
pub struct BoardSquare {
    pub coord: SquareCoord,
}

#[derive(Bundle, Reflect)]
pub struct BoardSquareBundle {
    board_square: BoardSquare,
    transform: Transform,
    sprite: Sprite,
}
impl BoardSquareBundle {
    pub fn new(x: u32, y: u32) -> Self {
        let x_location = (x as f32 - 5.0) * 0.1 + 0.05;
        let y_location = (y as f32 - 5.0) * 0.1 + 0.05;

        Self {
            board_square: BoardSquare { coord: SquareCoord(x, y) },
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

#[derive(Component, Reflect)]
pub struct Board;

#[derive(Bundle, Reflect)]
struct BoardBundle {
    board: Board,
    sprite: Sprite,
    transform: Transform,
}
impl BoardBundle {
    pub fn new() -> Self {
        Self {
            board: Board,
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
            .add_systems(Startup, setup_board);
    }
}

fn setup_board(mut commands: Commands) {
    commands.spawn(BoardBundle::new())
        .with_children(|board| {
            for row in 0..8 {
                for col in 0..8 {
                    board.spawn(BoardSquareBundle::new(row, col));
                }
            }
        });
}