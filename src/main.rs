use std::time::Duration;

mod board;
mod piece;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use piece::{Piece, PieceType};

const WINDOW_WIDTH: u32 = board::COLS * board::CELL_SIZE;
const WINDOW_HEIGHT: u32 = board::ROWS * board::CELL_SIZE;

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

    let board = board::Board::new();
    let current_piece = Piece::new(PieceType::T, 3, 0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        board.draw(&mut canvas)?;
        board.draw_piece(&mut canvas, &current_piece)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(1000 / 60));
    }

    Ok(())
}
