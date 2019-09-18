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
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition {
                rank: rank,
                file: file,
            })
        } else {
            None
        }
    }
    pub fn is_same_rank(&self, position: &ChessPosition) -> bool {
        self.rank == position.rank
    }

    pub fn is_same_file(&self, position: &ChessPosition) -> bool {
        self.file == position.file
    }

    pub fn is_same_diagonal(&self, position: &ChessPosition) -> bool {
        (self.rank - position.rank).abs() == (self.file - position.file).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.is_same_file(&other.position)
            || self.position.is_same_rank(&other.position)
            || self.position.is_same_diagonal(&other.position)
    }
}
