#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }
        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
        // todo!("Given the chess position {position:?}, construct a Queen struct.");
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || (i32::abs(self.position.rank - other.position.rank)
                == i32::abs(self.position.file - other.position.file))
    }
}
