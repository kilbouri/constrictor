mod io;
mod rendering;
mod scope_guard;

use constrictor_core::{
    math::{Direction, Vector2},
    models::{Board, SimulationParameterError, Snake, SnakeSimulation},
};
use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
};
use io::{EventStream, GameCommand};
use std::{
    error::Error,
    io::{Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{rendering::Renderable, scope_guard::ScopeGuard};

fn main() -> Result<(), Box<dyn Error>> {
    // Try and be a polite neighbour to the user. We're about to mess with their
    // terminal so we better at least try to clean up our own mess.
    let _restore_terminal = ScopeGuard::new(|| {
        // clean up with best effort
        _ = execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show);
        _ = terminal::disable_raw_mode();
    });

    // Grab handle to stdout and prepare the rendering environment
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut command_iter = EventStream::new()
        .filter_map(|e| e.ok())
        .filter_map(|e| GameCommand::try_from(e).ok());

    let mut sim = create_game(32, 32)?;

    sim.render(&mut stdout)?;

    while sim.result().is_none() {
        let frame_start = Instant::now();

        // Process input that has happened since last tick
        for command in command_iter.by_ref() {
            match command {
                GameCommand::Quit => sim.quit(),
                GameCommand::ChangeDirection(direction) => {
                    sim.change_player_move_direction(direction)
                }
            }
        }

        // Step simulation forward
        sim.advance();

        // Re-render
        queue!(
            &mut stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        sim.render(&mut stdout)?;
        stdout.flush()?;

        let frame_end = Instant::now();
        let frame_duration = frame_end - frame_start;
        let sleep_time = Duration::from_millis(75) - frame_duration;

        if sleep_time > Duration::ZERO {
            sleep(sleep_time);
        }
    }

    Ok(())
}

fn create_game(width: u16, height: u16) -> Result<SnakeSimulation, SimulationParameterError> {
    let w_i32: i32 = width.into();
    let h_i32: i32 = height.into();

    let center = Vector2 { x: w_i32, y: h_i32 } / 2;

    SnakeSimulation::new(
        Board::new((1, w_i32 + 1), (1, h_i32 + 1)),
        Snake::new(center.neighbour(Direction::Left, 3), Direction::Right),
        center.neighbour(Direction::Right, 3),
    )
}
