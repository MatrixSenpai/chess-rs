use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::board_plugin::{Board, BoardSquare};

#[derive(Debug, PartialEq, Copy, Clone, Component, Reflect)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
impl Into<usize> for PieceType {
    fn into(self) -> usize {
        match self {
            Self::King => 0,
            Self::Queen => 1,
            Self::Bishop => 2,
            Self::Knight => 3,
            Self::Rook => 4,
            Self::Pawn => 5,
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone, Component, Reflect)]
pub enum PieceSide {
    White,
    Black,
}
impl Into<usize> for PieceSide {
    fn into(self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => 1,
        }
    }
}
impl std::ops::Add<PieceSide> for PieceType {
    type Output = usize;

    fn add(self, rhs: PieceSide) -> Self::Output {
        <PieceSide as Into<usize>>::into(rhs) * 6 + <PieceType as Into<usize>>::into(self)
    }
}

#[derive(Component, Reflect)]
pub struct Piece {
    pub piece_type: PieceType,
    pub piece_side: PieceSide,
    pub identifier: usize,
}

#[derive(Bundle, Reflect)]
pub struct PieceBundle {
    piece: Piece,
    sprite: Sprite,
    transform: Transform,
}
impl PieceBundle {
    pub fn new(
        identifier: usize,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        piece_type: PieceType,
        piece_side: PieceSide
    ) -> Self {
        let index = piece_type + piece_side;

        Self {
            piece: Piece { identifier, piece_type, piece_side },
            sprite: Sprite::from_atlas_image(image, TextureAtlas {
                layout, index
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                scale: Vec3::new(0.002, 0.002, 0.0),
                ..default()
            },
        }
    }
}

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PieceType>()
            .register_type::<PieceSide>()
            .register_type::<Piece>()
            .add_systems(PostStartup, setup_pieces);
    }
}

fn setup_pieces(
    mut commands: Commands,
    board: Single<Entity, With<Board>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut board_entity = commands.entity(*board);

    let texture = asset_server.load("img/pieces.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(44), 6, 2, Some(UVec2::new(0, 2)), None
    );

    let layout = texture_atlas_layouts.add(layout);

    let pieces = [
        PieceType::Rook, PieceType::Knight, PieceType::Bishop,
        PieceType::Queen, PieceType::King,
        PieceType::Bishop, PieceType::Knight, PieceType::Rook,
    ];

    board_entity.with_children(|board| {
        for (s, side) in [PieceSide::White, PieceSide::Black].into_iter().enumerate() {
            for (i, piece) in pieces.into_iter().enumerate() {
                let id = s * 8 + i;
                board.spawn(PieceBundle::new(id, texture.clone(), layout.clone(), piece, side));
            }

            for i in 0..8 {
                let id = (s + 2) * 8 + i;
                board.spawn(PieceBundle::new(id, texture.clone(), layout.clone(), PieceType::Pawn, side));
            }
        }
    });
}