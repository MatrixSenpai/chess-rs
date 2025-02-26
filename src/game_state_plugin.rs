use bevy::prelude::*;
use crate::piece_plugin::*;
use crate::board_plugin::BoardSquare;

#[derive(Debug, Clone, Reflect)]
struct PieceInfo {
    piece_type: PieceType,
    piece_side: PieceSide,
    location: usize,
    identifier: usize,
}

#[derive(Resource, Reflect)]
struct GameState {
    pieces_locations: [Option<PieceInfo>; 32],
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            pieces_locations: [
                Some(PieceInfo { piece_type: PieceType::Rook, piece_side: PieceSide::White, location: 0, identifier: 0 }),
                Some(PieceInfo { piece_type: PieceType::Knight, piece_side: PieceSide::White, location: 1, identifier: 1 }),
                Some(PieceInfo { piece_type: PieceType::Bishop, piece_side: PieceSide::White, location: 2, identifier: 2 }),
                Some(PieceInfo { piece_type: PieceType::Queen, piece_side: PieceSide::White, location: 3, identifier: 3 }),
                Some(PieceInfo { piece_type: PieceType::King, piece_side: PieceSide::White, location: 4, identifier: 4 }),
                Some(PieceInfo { piece_type: PieceType::Bishop, piece_side: PieceSide::White, location: 5, identifier: 5 }),
                Some(PieceInfo { piece_type: PieceType::Knight, piece_side: PieceSide::White, location: 6, identifier: 6 }),
                Some(PieceInfo { piece_type: PieceType::Rook, piece_side: PieceSide::White, location: 7, identifier: 7 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 8, identifier: 16 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 9, identifier: 17 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 10, identifier: 18 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 11, identifier: 19 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 12, identifier: 20 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 13, identifier: 21 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 14, identifier: 22 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::White, location: 15, identifier: 23 }),

                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 48, identifier: 24 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 49, identifier: 25 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 50, identifier: 26 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 51, identifier: 27 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 52, identifier: 28 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 53, identifier: 29 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 54, identifier: 30 }),
                Some(PieceInfo { piece_type: PieceType::Pawn, piece_side: PieceSide::Black, location: 55, identifier: 31 }),
                Some(PieceInfo { piece_type: PieceType::Rook, piece_side: PieceSide::Black, location: 56, identifier: 8 }),
                Some(PieceInfo { piece_type: PieceType::Knight, piece_side: PieceSide::Black, location: 57, identifier: 9 }),
                Some(PieceInfo { piece_type: PieceType::Bishop, piece_side: PieceSide::Black, location: 58, identifier: 10 }),
                Some(PieceInfo { piece_type: PieceType::Queen, piece_side: PieceSide::Black, location: 59, identifier: 11 }),
                Some(PieceInfo { piece_type: PieceType::King, piece_side: PieceSide::Black, location: 60, identifier: 12 }),
                Some(PieceInfo { piece_type: PieceType::Bishop, piece_side: PieceSide::Black, location: 61, identifier: 13 }),
                Some(PieceInfo { piece_type: PieceType::Knight, piece_side: PieceSide::Black, location: 62, identifier: 14 }),
                Some(PieceInfo { piece_type: PieceType::Rook, piece_side: PieceSide::Black, location: 63, identifier: 15 }),
            ]
        }
    }
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PieceInfo>()
            .register_type::<GameState>()
            .init_resource::<GameState>()
            .add_systems(Update, position_pieces);
    }
}

fn position_pieces(
    mut pieces: Query<(&Piece, &mut Transform), Without<BoardSquare>>,
    squares: Query<(&BoardSquare, &Transform), Without<Piece>>,
    locations: Res<GameState>,
) {
    let piece_locations = locations.pieces_locations.iter()
        .filter_map(|v| v.clone())
        .collect::<Vec<PieceInfo>>();

    for piece_info in piece_locations.into_iter() {
        let x = piece_info.location % 8;
        let y = piece_info.location / 8;

        let (_, square_item) = match squares.iter().find(|(s, _)| s.coord.eq(&(x, y))) {
            Some(v) => v,
            None => {
                warn!("Cannot find square for location {x}/{y}");
                continue
            }
        };

        let (_, mut piece_item) = match pieces.iter_mut()
            .find(|(p, _)| p.identifier.eq(&piece_info.identifier))
        {
            Some(v) => v,
            None => {
                warn!("Cannot find piece for item {piece_info:?}");
                continue
            }
        };

        piece_item.translation = Vec3::new(square_item.translation.x - 0.01, square_item.translation.y, piece_item.translation.z);
    }
}