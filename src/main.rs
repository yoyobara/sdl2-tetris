use std::time::{Duration, Instant};

mod board;
mod piece;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use piece::{Piece, PieceType};

const WINDOW_WIDTH: u32 = board::COLS * board::CELL_SIZE;
const WINDOW_HEIGHT: u32 = board::ROWS * board::CELL_SIZE;
const FALL_INTERVAL: Duration = Duration::from_millis(500);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video
        .window("Tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut board = board::Board::new();
    let mut current_piece = Piece::new(PieceType::random(), 3, 0);
    let mut last_fall = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let moved = current_piece.moved_by(-1, 0);
                    if board.is_valid_position(&moved) {
                        current_piece = moved;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let moved = current_piece.moved_by(1, 0);
                    if board.is_valid_position(&moved) {
                        current_piece = moved;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let mut rotated = current_piece;
                    rotated.rotate_cw();
                    if board.is_valid_position(&rotated) {
                        current_piece = rotated;
                    }
                }
                _ => {}
            }
        }

        if last_fall.elapsed() >= FALL_INTERVAL {
            let fallen = current_piece.moved_by(0, 1);
            if board.is_valid_position(&fallen) {
                current_piece = fallen;
            } else {
                board.lock_piece(&current_piece);
                current_piece = Piece::new(PieceType::random(), 3, 0);
                if !board.is_valid_position(&current_piece) {
                    // No room for a new piece: start over.
                    board.clear();
                }
            }
            last_fall = Instant::now();
        }

        board.draw(&mut canvas)?;
        board.draw_piece(&mut canvas, &current_piece)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(1000 / 60));
    }

    Ok(())
}
