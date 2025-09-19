use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::hash_map::Entry;

use crate::math::Direction;
use crate::math::Vector2;

pub struct Snake {
    /// The direction the snake is currently facing.
    facing: Direction,

    /// The last direction the snake moved. This is important to prevent it
    /// being possible to make the snake reverse over itself by rotating
    /// [`Self::facing`] 90 degrees twice.
    last_move_direction: Direction,

    /// The points making up the snake's body.
    ///
    /// # Note
    /// You should avoid manual manipulation of this field because it can lead
    /// to divergence from [`Self::body_point_counts`].
    body: VecDeque<Vector2>,

    /// Map mirroring [`Self::body`] with the number of times a given point is
    /// covered by the snake. Allows accounting for self-intersection.
    ///
    /// # Note
    /// You should avoid manual manipulation of this field because it can lead
    /// to divergence from [`Self::body`].
    body_point_counts: HashMap<Vector2, usize>,
}

impl Snake {
    /// Creates a [`Snake`] facing `facing` with length 1 with head (and tail)
    /// located at `head_position`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// assert_eq!(snek.facing(), Direction::Right);
    /// assert_eq!(snek.len(), 1);
    /// assert_eq!(snek.head(), &Vector2 { x: 4, y: 2 });
    /// assert_eq!(snek.tail(), &Vector2 { x: 4, y: 2 });
    /// ```
    pub fn new(head_position: Vector2, facing: Direction) -> Self {
        let mut snek = Self {
            body: VecDeque::new(),
            body_point_counts: HashMap::new(),
            last_move_direction: facing,
            facing,
        };

        snek.push_head(head_position);

        snek
    }

    /// Gets the direction the [`Snake`] is facing.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// assert_eq!(snek.facing(), Direction::Right);
    /// ```
    pub fn facing(&self) -> Direction {
        self.facing
    }

    /// Gets the total length of the [`Snake`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// assert_eq!(snek.len(), 1);
    ///
    /// snek.advance(true);
    /// assert_eq!(snek.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// Returns `true` if the [`Snake`] has no body. Note that this always
    /// returns `true` if [`Snake::new`] is used, as that constructor
    /// ensures at least one body segment.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let snek = Snake::new(Vector2 {x: 4, y: 2 }, Direction::Right);
    /// assert!(!snek.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    /// Gets the position of the [`Snake`]'s head.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// assert_eq!(snek.head(), &Vector2{ x: 4, y: 2 });
    ///
    /// snek.advance(true);
    /// assert_eq!(snek.head(), &Vector2{ x: 5, y: 2 });
    ///
    /// snek.advance(false);
    /// assert_eq!(snek.head(), &Vector2{ x: 6, y: 2 });
    /// ```
    pub fn head(&self) -> &Vector2 {
        self.body.front().expect("snake is headless")
    }

    /// Returns an [`Iterator`] over the body of the [`Snake`], from head to
    /// tail.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// snek.advance(true);
    ///
    /// let mut iter = snek.body_iter();
    /// assert!(matches!(iter.next(), Some(Vector2{ x: 5, y: 2 })));
    /// assert!(matches!(iter.next(), Some(Vector2{ x: 4, y: 2 })));
    /// assert!(matches!(iter.next(), None));
    /// ```
    pub fn body_iter(&self) -> impl Iterator<Item = &Vector2> {
        self.body.iter()
    }

    /// Gets the position of the [`Snake`]'s tail.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// assert_eq!(snek.tail(), &Vector2{ x: 4, y: 2 });
    ///
    /// snek.advance(true);
    /// assert_eq!(snek.tail(), &Vector2{ x: 4, y: 2 });
    ///
    /// snek.advance(false);
    /// assert_eq!(snek.tail(), &Vector2{ x: 5, y: 2 });
    /// ```
    pub fn tail(&self) -> &Vector2 {
        self.body.back().expect("snake is tailless")
    }

    /// Returns whether or not the [`Snake`]'s body contains the provided
    /// [`Vector2`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    /// snek.advance(true);
    /// snek.advance(true);
    ///
    /// assert!(!snek.contains(&Vector2{ x: 7, y: 2 }));
    /// assert!(snek.contains(&Vector2{ x: 6, y: 2 }));
    /// assert!(snek.contains(&Vector2{ x: 5, y: 2 }));
    /// assert!(snek.contains(&Vector2{ x: 4, y: 2 }));
    /// assert!(!snek.contains(&Vector2{ x: 3, y: 2 }));
    /// ```
    pub fn contains(&self, point: &Vector2) -> bool {
        self.body_point_counts.contains_key(point)
    }

    /// Speculatively retrieve the [`Self::head`] of the [`Snake`] after the
    /// next call to [`Self::advance`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    ///
    /// let speculated_head = snek.next_head_position();
    ///
    /// snek.advance(false);
    ///
    /// assert_eq!(&speculated_head, snek.head());
    /// ```
    pub fn next_head_position(&self) -> Vector2 {
        self.head().neighbour(self.facing, 1)
    }

    /// Attempt to change the direction the [`Snake`] is moving. The direction
    /// is not changed and `false` is returned when the new direction would
    /// cause the [`Snake`] to reverse direction.
    ///
    /// The direction change does not take effect until [`Snake::advance`] is
    /// called. That is, if the [`Snake`] is facing [`Direction::Up`],
    /// calling `try_set_facing(Direction::Left)` then
    /// `try_set_facing(Direction::Down)` will still yield `false` for the
    /// latter.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Up);
    ///
    /// snek.advance(false);
    ///
    /// // Still invalid because the snake moved in the direction it was facing before.
    /// assert!(!snek.try_set_facing(Direction::Down));
    ///
    /// // Valid as it is perpendicular to the direction the snake last moved.
    /// assert!(snek.try_set_facing(Direction::Right));
    ///
    /// // Should remain invalid, as the snake has not actually advanced Right yet.
    /// assert!(!snek.try_set_facing(Direction::Down));
    ///
    /// snek.advance(false);
    ///
    /// // Should now become valid, as the snake has advanced Right.
    /// assert!(snek.try_set_facing(Direction::Down));
    /// ```
    pub fn try_set_facing(&mut self, new_direction: Direction) -> bool {
        if new_direction != self.last_move_direction.flip() {
            self.facing = new_direction;
            true
        } else {
            false
        }
    }

    /// Advances the [`Snake`] by a single step. Each step moves the head in the
    /// direction of `self.facing` by one and drops the tail to  maintain length
    /// (unless the [`Snake`] `consumed_food`).
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    /// use constrictor_core::models::Snake;
    ///
    /// let mut snek = Snake::new(Vector2 { x: 4, y: 2 }, Direction::Right);
    ///
    /// snek.advance(false);
    /// assert_eq!(snek.len(), 1);
    /// assert_eq!(snek.head(), &Vector2 { x: 5, y: 2 });
    /// assert_eq!(snek.tail(), &Vector2 { x: 5, y: 2 });
    ///
    /// snek.advance(true);
    /// assert_eq!(snek.len(), 2);
    /// assert_eq!(snek.head(), &Vector2 { x: 6, y: 2 });
    /// assert_eq!(snek.tail(), &Vector2 { x: 5, y: 2 });
    /// ```
    pub fn advance(&mut self, consumed_food: bool) {
        // Though it should never be valid, do this first in case len() == 1
        let new_head = self.next_head_position();

        // Dropping the tail first ensures we can avoid pointless collection growth
        if !consumed_food {
            _ = self.pop_tail();
        }

        self.push_head(new_head);
        self.last_move_direction = self.facing
    }

    /// Push a new head onto the snake.
    ///
    /// # Note
    /// You should avoid manual manipulation of [`Self::body`] and
    /// [`Self::body_point_counts`] because it can lead to the two diverging.
    fn push_head(&mut self, head: Vector2) {
        self.body.push_front(head);
        *self.body_point_counts.entry(head).or_insert(0) += 1;
    }

    /// Pop the tail from the snake.
    ///
    /// # Note
    /// You should avoid manual manipulation of [`Self::body`] and
    /// [`Self::body_point_counts`] because it can lead to the two diverging.
    fn pop_tail(&mut self) -> Option<Vector2> {
        let old_tail = self.body.pop_back()?;

        match self.body_point_counts.entry(old_tail) {
            Entry::Occupied(entry) if (*entry.get() <= 1) => _ = entry.remove_entry(),
            Entry::Occupied(mut entry) => *entry.get_mut() -= 1,
            Entry::Vacant(_) => {
                unreachable!("Snake::body and Snake::body_point_counts have diverged!",)
            }
        }

        Some(old_tail)
    }
}
