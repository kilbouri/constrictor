use constrictor_core::math::Direction;
use crossterm::event::{Event, KeyCode, KeyEvent};

/// The types of commands a user (or automated system) can input into the game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameCommand {
    /// Command to change the direction of the snake.
    ChangeDirection(Direction),

    /// Command to quit the game.
    Quit,
}

impl TryFrom<Event> for GameCommand {
    type Error = Event;

    /// Parses the [`Event`] into either [`Ok<GameCommand>`] if the event maps
    /// to a [`GameCommand`], otherwise to [`Err<Event>`] to allow for
    /// further parsing of the original event.
    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Key(e) => e.try_into().map_err(|_| Event::Key(e)),
            e => Err(e),
        }
    }
}

impl TryFrom<KeyEvent> for GameCommand {
    type Error = KeyEvent;

    /// Parses the [`KeyEvent`] into either [`Ok<GameCommand>`] if event maps to
    /// a [`GameCommand`], otherwise to [`Err<KeyEvent>`] to allow for
    /// further parsing of the original key.
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        match value.code {
            KeyCode::Char('w') | KeyCode::Up => Ok(GameCommand::ChangeDirection(Direction::Up)),
            KeyCode::Char('a') | KeyCode::Left => Ok(GameCommand::ChangeDirection(Direction::Left)),
            KeyCode::Char('s') | KeyCode::Down => Ok(GameCommand::ChangeDirection(Direction::Down)),
            KeyCode::Char('d') | KeyCode::Right => {
                Ok(GameCommand::ChangeDirection(Direction::Right))
            }
            KeyCode::Char('q') => Ok(GameCommand::Quit),
            _ => Err(value),
        }
    }
}
