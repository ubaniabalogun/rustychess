pub struct Chess {
    state: GameState,
    board: Board,
}

impl Chess {
    pub fn new() -> Self {
        Self {
            state: GameState::Turn(Player::White),
            board: Board::new(),
        }
    }

    pub fn handle(&mut self, moove: Move) -> Effect {
        if let Move::Resign = moove {
            // do a bunch of stuff to resign the game
            todo!("Implement shut down for Move::Resign")
        }

        match &self.state {
            GameState::Check => todo!(),
            GameState::Checkmate(player) => todo!(),
            GameState::Stalemate => todo!(),
            GameState::Turn(player) => todo!(),
            GameState::PawnPromotion(player) => todo!(),
        }

        todo!("This hasn't been implemented yet!")
    }
}

#[derive(Debug)]
pub enum Player {
    White,
    Black,
}

struct Board {}

impl Board {
    fn new() -> Self {
        Self {}
    }
}

enum GameState {
    Check,
    Checkmate(Player),
    Stalemate,
    Turn(Player),
    PawnPromotion(Player),
}

// Spot(rank, file)
type Spot = (u32, u32);

#[derive(Debug)]
pub enum Move {
    Piece {
        player: Player,
        from: Spot,
        to: Spot,
    },
    Resign,
}

// The effect a move had
pub struct Effect {}
