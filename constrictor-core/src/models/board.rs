use std::ops::Range;

use crate::math::Vector2;

pub struct Board {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Board {
    /// Constructs a new [`Board`] with from inclusive lower and exclusive upper
    /// bounds on each `x` and `y`.
    ///
    /// Panics if `x.0 >= x.1` or `y.0 >= y.1`.
    ///
    /// ```
    /// use constrictor_core::models::Board;
    ///
    /// assert_eq!(Board::new((-10, 10), (-5, 5)).x_range(), -10..10);
    /// ```
    pub fn new(x: (i32, i32), y: (i32, i32)) -> Self {
        assert!(x.0 < x.1);
        assert!(y.0 < y.1);

        Self {
            min_x: x.0.min(x.1),
            max_x: x.0.max(x.1),
            min_y: y.0.min(y.1),
            max_y: y.0.max(y.1),
        }
    }

    /// Returns the width of this [`Board`].
    ///
    /// ```
    /// use constrictor_core::models::Board;
    ///
    /// assert_eq!(Board::new((-10, 10), (-5, 5)).width(), 20)
    /// ```
    pub const fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    /// Returns the height of this [`Board`].
    ///
    /// ```
    /// use constrictor_core::models::Board;
    ///
    /// assert_eq!(Board::new((-10, 10), (-5, 5)).height(), 10)
    /// ```
    pub const fn height(&self) -> i32 {
        self.max_y - self.min_y
    }

    /// Returns the [`Range`] of valid x-coordinates for this [`Board`].
    ///
    /// ```
    /// use constrictor_core::models::Board;
    ///
    /// assert_eq!(Board::new((-10, 10), (-5, 5)).x_range(), -10..10)
    /// ```
    pub const fn x_range(&self) -> Range<i32> {
        self.min_x..self.max_x
    }

    /// Returns the [`Range`] of valid y-coordinates for this [`Board`].
    ///
    /// ```
    /// use constrictor_core::models::Board;
    ///
    /// assert_eq!(Board::new((-10, 10), (-5, 5)).y_range(), -5..5)
    /// ```
    pub const fn y_range(&self) -> Range<i32> {
        self.min_y..self.max_y
    }

    /// Returns an [`Iterator<Item = Vector2>`] over the cells in the board.
    pub fn cell_iter(&self) -> impl Iterator<Item = Vector2> {
        self.y_range()
            .flat_map(|y| self.x_range().map(move |x| Vector2 { x, y }))
    }

    /// Determines whether or not `point` is contained within the [`Board`].
    ///
    /// ```
    /// use constrictor_core::models::Board;
    /// use constrictor_core::math::Vector2;
    ///
    /// assert!(Board::new((-10, 10), (-5, 5)).contains(&Vector2 { x: 9, y: 4 }));
    /// assert!(!Board::new((-10, 10), (-5, 5)).contains(&Vector2 { x: 10, y: 5 }));
    /// ```
    pub fn contains(&self, point: &Vector2) -> bool {
        self.x_range().contains(&point.x) && self.y_range().contains(&point.y)
    }

    /// Generates a random free cell according to `is_taken`. Returns [`None`]
    /// if no free cell could be found.
    ///
    /// ### Note:
    /// If, and only if, `taken_cell_count` is guaranteed to be the same number
    /// of cells as `is_taken` would find when run on the entire board, the
    /// following additional guarantees hold:
    /// - if at least one free cell exists, [`None`] will never be returned, and
    /// - the free cell is chosen with uniformity, as specified by
    ///   [`rand::distr::Uniform`]
    pub fn random_free_cell<F: Fn(&Vector2) -> bool>(
        &self,
        taken_cell_count: usize,
        is_taken: F,
    ) -> Option<Vector2> {
        let total_cells = (self.width() as usize) * (self.height() as usize);
        let free_cells = total_cells - taken_cell_count;

        let target_cell = rand::random_range(0..free_cells);

        self.cell_iter()
            .filter(|cell| !is_taken(cell))
            .skip(target_cell)
            .next()
    }
}
