use crate::position::Position;

pub struct Syncer {
    pub received_position: Position,
    pub committed_position: Position,
}
