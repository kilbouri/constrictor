use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::math::Direction;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Default)]
pub struct Vector2<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Sub<Output = T>> Vector2<T> {
    /// Gets a [`Vector2`] shifted by `magnitude` in `direction`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    ///
    /// let a = Vector2::default();
    ///
    /// assert_eq!(a.neighbour(Direction::Up, 5), Vector2 { x: 0, y: 5 });
    /// assert_eq!(a.neighbour(Direction::Down, 5), Vector2 { x: 0, y: -5 });
    /// assert_eq!(a.neighbour(Direction::Right, 5), Vector2 { x: 5, y: 0 });
    /// assert_eq!(a.neighbour(Direction::Left, 5), Vector2 { x: -5, y: 0 });
    /// ```
    pub fn neighbour(self, direction: Direction, magnitude: T) -> Self {
        match direction {
            Direction::Up => Self {
                y: self.y + magnitude,
                ..self
            },
            Direction::Down => Self {
                y: self.y - magnitude,
                ..self
            },
            Direction::Left => Self {
                x: self.x - magnitude,
                ..self
            },
            Direction::Right => Self {
                x: self.x + magnitude,
                ..self
            },
        }
    }
}

impl<T: AddAssign + SubAssign> Vector2<T> {
    /// Moves `self` by `magnitude` in `direction`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::{Direction, Vector2};
    ///
    /// let mut a = Vector2 { x: 0, y: 0 };
    ///
    /// a.move_in(Direction::Up, 5);
    /// assert_eq!(a, Vector2 { x: 0, y: 5 });
    ///
    /// a.move_in(Direction::Down, 5);
    /// assert_eq!(a, Vector2 { x: 0, y: 0 });
    ///
    /// a.move_in(Direction::Left, 5);
    /// assert_eq!(a, Vector2 { x: -5, y: 0 });
    ///
    /// a.move_in(Direction::Right, 5);
    /// assert_eq!(a, Vector2 { x: 0, y: 0 });
    /// ```
    pub fn move_in(&mut self, direction: Direction, magnitude: T) {
        match direction {
            Direction::Up => self.y += magnitude,
            Direction::Down => self.y -= magnitude,
            Direction::Left => self.x -= magnitude,
            Direction::Right => self.x += magnitude,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Vector2<T>;

    /// Negates the [`Vector2<T>`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let a = Vector2 { x: 1, y: 10 };
    ///
    /// assert_eq!(-a, Vector2 { x: -1, y: -10 })
    /// ```
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    /// Element-wise addition of two [`Vector2<T>`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let a = Vector2 { x: 1, y: 10 };
    /// let b = Vector2 { x: 5, y: 2 };
    ///
    /// assert_eq!(a + b, Vector2 { x: 6, y: 12 })
    /// ```
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    /// Performs in-place element-wise addition of `rhs` into `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let mut a = Vector2 { x: 1, y: 10 };
    ///
    /// a += Vector2 { x: 5, y: 2 };
    ///
    ///  assert_eq!(a, Vector2 { x: 6, y: 12 })
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    /// Performs element-wise subtraction of two [`Vector2<T>`].
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let a = Vector2 { x: 1, y: 10 };
    /// let b = Vector2 { x: 5, y: 2 };
    ///
    /// assert_eq!(a - b, Vector2 { x: -4, y: 8 })
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    /// Performs in-place element-wise subtraction of two `rhs` into `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let mut a = Vector2 { x: 1, y: 10 };
    ///
    /// a -= Vector2 { x: 5, y: 2 };
    ///
    /// assert_eq!(a, Vector2 { x: -4, y: 8 })
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    /// Performs element-wise scalar multiplication of `rhs` on `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let a = Vector2 { x: 1, y: 2 };
    ///
    /// assert_eq!(a * 5, Vector2 { x: 5, y: 10 });
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    /// Performs in-place element-wise scalar multiplication of `rhs` on `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let mut a = Vector2 { x: 1, y: 2 };
    ///
    /// a *= 5;
    ///
    /// assert_eq!(a, Vector2 { x: 5, y: 10 });
    /// ```
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    /// Performs element-wise scalar division of `rhs` on `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let mut a = Vector2 { x: 5, y: 10 };
    ///
    /// assert_eq!(a / 5, Vector2 { x: 1, y: 2 });
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    /// Performs in-place element-wise scalar division of `rhs` on `self`.
    ///
    /// # Example
    /// ```
    /// use constrictor_core::math::Vector2;
    ///
    /// let mut a = Vector2 { x: 5, y: 10 };
    ///
    /// a /= 5;
    ///
    /// assert_eq!(a, Vector2 { x: 1, y: 2 });
    /// ```
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
