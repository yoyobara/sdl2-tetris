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

    /// The piece's 4 occupied cells in board coordinates.
    pub fn cells(&self) -> [(i32, i32); 4] {
        let shape = &SHAPES[self.kind as usize][self.rotation as usize];
        shape.map(|(row, col)| (self.x + col as i32, self.y + row as i32))
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }
}
