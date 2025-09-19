use std::{error::Error, io::Write, iter, num::TryFromIntError};

use constrictor_core::{
    math::Vector2,
    models::{Board, Snake, SnakeSimulation},
};
use crossterm::{
    cursor, queue,
    style::{self, Color},
};

trait TryToScreen<S, E> {
    fn try_to_screen(&self) -> Result<S, E>;
}

impl TryToScreen<Vector2<u16>, TryFromIntError> for Vector2 {
    fn try_to_screen(&self) -> Result<Vector2<u16>, TryFromIntError> {
        let x: u16 = (self.x * 2 - 1).try_into()?;
        let y: u16 = (self.y).try_into()?;

        Ok(Vector2 { x, y })
    }
}

pub trait Renderable {
    fn render<W: Write>(&self, stream: &mut W) -> Result<(), Box<dyn Error>>;
}

impl Renderable for SnakeSimulation {
    fn render<W: Write>(&self, stream: &mut W) -> Result<(), Box<dyn Error>> {
        const FOOD: &str = "╺╸";

        self.board().render(stream)?;
        self.snake().render(stream)?;

        let food_pos = self.food_position().try_to_screen()?;

        queue!(
            stream,
            cursor::MoveTo(food_pos.x, food_pos.y),
            style::SetForegroundColor(Color::Red),
            style::Print(FOOD)
        )?;

        Ok(())
    }
}

impl Renderable for Board {
    fn render<W: Write>(&self, stream: &mut W) -> Result<(), Box<dyn Error>> {
        const TOP_LEFT_CORNER: char = '╔';
        const TOP_RIGHT_CORNER: char = '╗';
        const BOTTOM_LEFT_CORNER: char = '╚';
        const BOTTOM_RIGHT_CORNER: char = '╝';
        const VERTICAL_WALL: char = '║';
        const HORIZONTAL_WALL: char = '═';

        let w_u16: u16 = (self.width() * 2).try_into()?;
        let horizontal_bars = iter::repeat_n(HORIZONTAL_WALL, w_u16 as usize).collect::<String>();

        queue!(
            stream,
            style::SetForegroundColor(Color::DarkGrey),
            style::Print(&TOP_LEFT_CORNER),
            style::Print(&horizontal_bars),
            style::Print(&TOP_RIGHT_CORNER),
            cursor::MoveToNextLine(1)
        )?;

        for _ in 0..self.height() {
            queue!(
                stream,
                style::Print(VERTICAL_WALL),
                cursor::MoveRight(w_u16),
                style::Print(VERTICAL_WALL),
                cursor::MoveToNextLine(1)
            )?;
        }

        queue!(
            stream,
            style::Print(&BOTTOM_LEFT_CORNER),
            style::Print(&horizontal_bars),
            style::Print(&BOTTOM_RIGHT_CORNER)
        )?;

        Ok(())
    }
}

impl Renderable for Snake {
    fn render<W: Write>(&self, stream: &mut W) -> Result<(), Box<dyn Error>> {
        const SNAKE_HEAD: &str = "██";
        const SNAKE_BODY: &str = "░░";

        let head_pos = self.head().try_to_screen()?;

        queue!(
            stream,
            cursor::MoveTo(head_pos.x, head_pos.y),
            style::SetForegroundColor(Color::Green),
            style::Print(SNAKE_HEAD)
        )?;

        for segment in self.body_iter().skip(1) {
            let body_pos = segment.try_to_screen()?;

            queue!(
                stream,
                cursor::MoveTo(body_pos.x, body_pos.y),
                style::SetForegroundColor(Color::Green),
                style::Print(SNAKE_BODY)
            )?;
        }

        Ok(())
    }
}
