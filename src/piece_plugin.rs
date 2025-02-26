use bevy::prelude::*;

#[derive(Copy, Clone, Component, Reflect)]
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
#[derive(Copy, Clone, Component, Reflect)]
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

#[derive(Bundle, Reflect)]
pub struct Piece {
    piece_type: PieceType,
    piece_side: PieceSide,
    sprite: Sprite,
    transform: Transform,
}
impl Piece {
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>, piece_type: PieceType, piece_side: PieceSide) -> Self {
        let index = piece_type + piece_side;

        Self {
            piece_type, piece_side,
            sprite: Sprite::from_atlas_image(image, TextureAtlas {
                layout, index
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                scale: Vec3::new(1.0, 1.0, 0.0),
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("img/pieces.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(44), 6, 2, Some(UVec2::new(0, 2)), None
    );

    let layout = texture_atlas_layouts.add(layout);

    commands.spawn(Piece::new(texture.clone(), layout.clone(), PieceType::Queen, PieceSide::Black));
}