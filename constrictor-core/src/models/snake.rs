use std::collections::VecDeque;

use crate::math::Direction;
use crate::math::Vector2;

pub struct Snake {
    /// The direction the snake is currently facing
    pub facing: Direction,

    /// The points making up the snake's body.
    body_points: VecDeque<Vector2>,
}

impl Snake {
    /// Creates a [`Snake`] with `length` length facing `facing` with head located at
    /// `head_position`. `length` must be at least `2`.
    pub fn new(head_position: Vector2, length: usize, facing: Direction) -> Self {
        assert!(
            length >= 2,
            "snake length must be at least 2 to have head and tail"
        );

        let mut snek = Self {
            body_points: VecDeque::with_capacity(1 + length),
            facing,
        };

        // Insert each of the body positions
        let mut body_point = head_position;
        for _ in 0..length {
            snek.body_points.push_back(body_point);
            body_point.move_in(facing.flip(), 1);
        }

        snek
    }

    /// Gets the total length of the [`Snake`].
    pub fn len(&self) -> usize {
        self.body_points.len()
    }

    /// Gets the position of the [`Snake`]'s head.
    pub fn head(&self) -> Vector2 {
        self.body_points.front().expect("snake is headless").clone()
    }

    /// Gets the position of the [`Snake`]'s tail.
    pub fn tail(&self) -> Vector2 {
        self.body_points.back().expect("snake is tailless").clone()
    }

    /// Advances the [`Snake`] by a single step. Each step moves the head in the direction of
    /// `self.facing` by one and drops drops the tail to maintain length (unless the [`Snake`]
    /// `consumed_food`).
    pub fn advance(&mut self, consumed_food: bool) {
        // Though it should never be valid, do this first in case len() == 1
        let new_head = self.head().neighbour(self.facing, 1);

        // Dropping the tail first ensures we can avoid pointless deque growth
        if !consumed_food {
            self.body_points.pop_back();
        }

        self.body_points.push_front(new_head);
    }

    /// Shifts the [`Snake`] by a given `offset`.
    pub fn shift(&mut self, offset: Vector2) {
        for point in self.body_points.iter_mut() {
            *point += offset;
        }
    }

    /// Gets the top-left and bottom-right corners of the smallest bounding box the [`Snake`]
    /// can fit within.
    pub fn bounds(&self) -> (Vector2, Vector2) {
        let top_left = self
            .body_points
            .iter()
            .copied()
            .reduce(|acc, point| Vector2 {
                x: acc.x.min(point.x),
                y: acc.y.max(point.y),
            })
            .expect("snake has no body points");

        let bottom_right = self
            .body_points
            .iter()
            .copied()
            .reduce(|acc, point| Vector2 {
                x: acc.x.max(point.x),
                y: acc.y.min(point.y),
            })
            .expect("snake has no body points");

        (top_left, bottom_right)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{Direction, Vector2},
        models::Snake,
    };

    #[test]
    fn new_snake_has_head_and_body() {
        let snek = Snake::new(Vector2::default(), 2, Direction::Up);

        assert_eq!(snek.len(), 2);
        assert_eq!(snek.head(), Vector2 { x: 0, y: 0 });
        assert_eq!(snek.tail(), Vector2 { x: 0, y: -1 });

        let snek2 = Snake::new(Vector2 { x: 5, y: 5 }, 5, Direction::Left);
        assert_eq!(snek2.len(), 5);
        assert_eq!(snek2.head(), Vector2 { x: 5, y: 5 });
        assert_eq!(snek2.tail(), Vector2 { x: 9, y: 5 });
    }

    #[test]
    fn new_snake_has_contiguous_body_points() {
        let snek = Snake::new(Vector2::default(), 30, Direction::Right);
        let mut pair_iter = snek.body_points.iter().skip(1).zip(snek.body_points.iter());

        assert!(pair_iter.all(|(&a, &b)| {
            let diff = a - b;
            let dx = diff.x.abs();
            let dy = diff.y.abs();

            (dx <= 1) && (dy <= 1) && (dx ^ dy == 1)
        }));
    }

    #[test]
    fn snake_can_move_forward() {
        let mut snek = Snake::new(Vector2::default(), 3, Direction::Up);

        snek.advance(false);

        assert_eq!(
            snek.body_points,
            vec![
                Vector2 { x: 0, y: 1 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: -1 }
            ]
        );
    }

    #[test]
    fn snake_can_move_around_corners() {
        let mut snek = Snake::new(Vector2::default(), 3, Direction::Up);

        // Corner move check
        snek.facing = Direction::Right;
        snek.advance(false);

        assert_eq!(
            snek.body_points,
            vec![
                Vector2 { x: 1, y: 0 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: -1 }
            ]
        );
    }

    #[test]
    fn shift_works() {
        let mut snek = Snake::new(Vector2::default(), 3, Direction::Right);
        snek.shift(Vector2 { x: 10, y: -10 });
        assert_eq!(
            snek.body_points,
            vec![
                Vector2 { x: 10, y: -10 },
                Vector2 { x: 9, y: -10 },
                Vector2 { x: 8, y: -10 }
            ]
        )
    }

    #[test]
    fn bounds_works() {
        let mut snek = Snake::new(Vector2::default(), 4, Direction::Up);
        snek.facing = Direction::Right;
        snek.advance(false);
        snek.advance(false);

        assert_eq!(
            snek.bounds(),
            (Vector2 { x: 0, y: 0 }, Vector2 { x: 2, y: -1 })
        );

        snek = Snake::new(Vector2::default(), 4, Direction::Down);
        snek.facing = Direction::Left;
        snek.advance(false);
        snek.advance(false);

        assert_eq!(
            snek.bounds(),
            (Vector2 { x: -2, y: 1 }, Vector2 { x: 0, y: 0 })
        )
    }
}
