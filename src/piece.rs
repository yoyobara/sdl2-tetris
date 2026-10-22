use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    pub fn color(&self) -> Color {
        match self {
            PieceType::I => Color::RGB(0, 240, 240),
            PieceType::O => Color::RGB(240, 240, 0),
            PieceType::T => Color::RGB(160, 0, 240),
            PieceType::S => Color::RGB(0, 240, 0),
            PieceType::Z => Color::RGB(240, 0, 0),
            PieceType::J => Color::RGB(0, 0, 240),
            PieceType::L => Color::RGB(240, 160, 0),
        }
    }
}

const PIECE_TYPES: [PieceType; 7] = [
    PieceType::I,
    PieceType::O,
    PieceType::T,
    PieceType::S,
    PieceType::Z,
    PieceType::J,
    PieceType::L,
];

impl PieceType {
    pub fn random() -> PieceType {
        use rand::seq::IndexedRandom;

        *PIECE_TYPES.choose(&mut rand::rng()).unwrap()
    }
}

// SRS rotation states, per piece type: 4 rotations x 4 cells, each cell as (row, col)
// in a 4x4 bounding box (row 0 = top).
const SHAPES: [[[(i8, i8); 4]; 4]; 7] = [
    // I
    [
        [(1, 0), (1, 1), (1, 2), (1, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 1), (1, 1), (2, 1), (3, 1)],
    ],
    // O
    [
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (1, 2)],
    ],
    // T
    [
        [(0, 1), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (2, 1)],
    ],
    // S
    [
        [(0, 1), (0, 2), (1, 0), (1, 1)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 1), (1, 2), (2, 0), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
    ],
    // Z
    [
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        [(0, 2), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 1), (1, 0), (1, 1), (2, 0)],
    ],
    // J
    [
        [(0, 0), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 0), (2, 1)],
    ],
    // L
    [
        [(0, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (1, 2), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (2, 1)],
    ],
];

#[derive(Clone, Copy)]
pub struct Piece {
    kind: PieceType,
    rotation: u8,
    x: i32,
    y: i32,
}

impl Piece {
    pub fn new(kind: PieceType, x: i32, y: i32) -> Self {
        Piece {
            kind,
            rotation: 0,
            x,
            y,
        }
    }

    pub fn kind(&self) -> PieceType {
        self.kind
    }

    pub fn rotation(&self) -> u8 {
        self.rotation
    }

    /// The piece's 4 occupied cells in board coordinates.
    pub fn cells(&self) -> [(i32, i32); 4] {
        let shape = &SHAPES[self.kind as usize][self.rotation as usize];
        shape.map(|(row, col)| (self.x + col as i32, self.y + row as i32))
    }

    pub fn moved_by(&self, dx: i32, dy: i32) -> Piece {
        Piece {
            x: self.x + dx,
            y: self.y + dy,
            ..*self
        }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }
}

// SRS wall kick offsets, tested in order, for each rotation transition.
// Offsets are given in the standard SRS convention (+x right, +y up); since
// this board's row coordinate grows downward, the y component must be
// negated when applied to a `Piece`'s (x, y).
//
// Transition order: 0->R, R->0, R->2, 2->R, 2->L, L->2, L->0, 0->L.
const JLSTZ_KICKS: [[(i32, i32); 5]; 8] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

const I_KICKS: [[(i32, i32); 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];

fn kick_transition_index(from: u8, to: u8) -> usize {
    match (from, to) {
        (0, 1) => 0,
        (1, 0) => 1,
        (1, 2) => 2,
        (2, 1) => 3,
        (2, 3) => 4,
        (3, 2) => 5,
        (3, 0) => 6,
        (0, 3) => 7,
        _ => unreachable!("not a single-step rotation: {from} -> {to}"),
    }
}

/// The wall kick offsets to try, in order, for rotating `kind` from `from` to `to`.
/// Offsets are already converted to this board's (x, row) convention.
pub fn wall_kicks(kind: PieceType, from: u8, to: u8) -> [(i32, i32); 5] {
    if kind == PieceType::O {
        return [(0, 0); 5];
    }

    let table = if kind == PieceType::I {
        &I_KICKS
    } else {
        &JLSTZ_KICKS
    };
    table[kick_transition_index(from, to)].map(|(dx, dy)| (dx, -dy))
}
