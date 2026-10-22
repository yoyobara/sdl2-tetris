use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::piece::{Piece, PieceType};

pub const COLS: u32 = 10;
pub const ROWS: u32 = 20;
pub const CELL_SIZE: u32 = 40;

const BACKGROUND_COLOR: Color = Color::RGB(15, 15, 20);
const GRID_COLOR: Color = Color::RGB(60, 60, 70);

pub struct Board {
    cells: Vec<Vec<Option<PieceType>>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: vec![vec![None; COLS as usize]; ROWS as usize],
        }
    }

    pub fn is_valid_position(&self, piece: &Piece) -> bool {
        piece.cells().iter().all(|&(col, row)| {
            col >= 0
                && col < COLS as i32
                && row >= 0
                && row < ROWS as i32
                && self.cells[row as usize][col as usize].is_none()
        })
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let width = (COLS * CELL_SIZE) as i32;
        let height = (ROWS * CELL_SIZE) as i32;

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.fill_rect(Rect::new(0, 0, width as u32, height as u32))?;

        for (row, cells_in_row) in self.cells.iter().enumerate() {
            for (col, cell) in cells_in_row.iter().enumerate() {
                if let Some(kind) = cell {
                    canvas.set_draw_color(kind.color());
                    let rect = Rect::new(
                        col as i32 * CELL_SIZE as i32,
                        row as i32 * CELL_SIZE as i32,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                    canvas.fill_rect(rect)?;
                }
            }
        }

        canvas.set_draw_color(GRID_COLOR);
        for col in 0..=COLS {
            let x = (col * CELL_SIZE) as i32;
            canvas.draw_line((x, 0), (x, height))?;
        }
        for row in 0..=ROWS {
            let y = (row * CELL_SIZE) as i32;
            canvas.draw_line((0, y), (width, y))?;
        }

        Ok(())
    }

    pub fn draw_piece(&self, canvas: &mut WindowCanvas, piece: &Piece) -> Result<(), String> {
        canvas.set_draw_color(piece.kind().color());
        for (col, row) in piece.cells() {
            let rect = Rect::new(
                col * CELL_SIZE as i32,
                row * CELL_SIZE as i32,
                CELL_SIZE,
                CELL_SIZE,
            );
            canvas.fill_rect(rect)?;
        }

        Ok(())
    }
}
