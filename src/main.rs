use game::{Chess, Effect, Move, Player};

mod game;

fn main() {
    let chess = Chess::new();
    let first_move = Move::Piece {
        player: Player::White,
        from: (4, 1),
        to: (4, 3),
    };

    let effect = chess.handle(first_move);
}
