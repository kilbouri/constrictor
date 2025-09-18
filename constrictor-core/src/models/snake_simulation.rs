use crate::{
    math::{Direction, Vector2},
    models::{Board, Snake},
};

/// Describes the outcome of a [`SnakeSimulation`].
#[derive(PartialEq, Eq, Debug)]
pub enum SimulationResult {
    /// The snake died for the specified reason.
    Died(DeathReason),

    // The simulation is complete. There is no more food to consume.
    Won,
}

/// Describes the reason a [`SnakeSimulation`] ended with
/// [`SimulationResult::Died`].
#[derive(PartialEq, Eq, Debug)]
pub enum DeathReason {
    /// The [`Snake`] collided with an edge of the [`Board`].
    HitWall,

    /// The [`Snake`] collided with itself.
    HitSelf,
}

/// Represents a virtual game of Classic Snake. The rules are:
/// - the [`Snake`] cannot intersect itself
/// - the [`Snake`] cannot go out of bounds, and does not wrap around when
///   hitting an edge
/// - the [`Snake`] grows in length by 1 when consuming food
pub struct SnakeSimulation {
    /// The board the game is taking place on.
    board: Board,

    /// The player's snake.
    snake: Snake,

    /// The position of the food.
    food_position: Vector2,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SimulationParameterError {
    /// One or more parts of the provided [`Snake`] is out of the bounds of the
    /// provided [`Board`].
    SnakeOutOfBounds,

    /// The provided [`Vector2`] for the position of the food is out of the
    /// bounds of the provided [`Board`].
    FoodOutOfBounds,
}

impl SnakeSimulation {
    /// Create a new [`SnakeSimulation`] from a [`Board`] and [`Snake`] with the
    /// food positioned at the position [`Vector2`].
    pub fn new(
        board: Board,
        snake: Snake,
        food_position: Vector2,
    ) -> Result<Self, SimulationParameterError> {
        if !board.contains(&food_position) {
            return Err(SimulationParameterError::FoodOutOfBounds);
        }

        if !snake.body_iter().all(|cell| board.contains(cell)) {
            return Err(SimulationParameterError::SnakeOutOfBounds);
        }

        Ok(Self {
            board,
            snake,
            food_position,
        })
    }

    /// Hook to change the player's movement direction. Intended to be called
    /// within input handling logic.
    pub fn change_player_move_direction(&mut self, new_direction: Direction) {
        self.snake.try_set_facing(new_direction);
    }

    /// Step the simulation forward by one step. The player's [`Snake`] will
    /// move, possibly consuming food and growing. If the player wins or
    /// dies, [`Some<SimulationResult>`] is returned accordingly. Otherwise,
    /// [`None`] is returned.
    pub fn advance(&mut self) -> Option<SimulationResult> {
        let speculative_head = self.snake.next_head_position();

        if !self.board.contains(&speculative_head) {
            return Some(SimulationResult::Died(DeathReason::HitWall));
        }

        // Check if we're about to run into ourselves
        let snake_will_hit_food = speculative_head == self.food_position;
        let snake_will_hit_tail = &speculative_head == self.snake.tail();

        if self.snake.contains(&speculative_head) && (!snake_will_hit_tail || snake_will_hit_food) {
            return Some(SimulationResult::Died(DeathReason::HitSelf));
        }

        // The snake should advance before we respawn the food, else it is possible for
        // the food to spawn exactly where the head ends up. This puts us in an invalid
        // state where the snake is on top of the food.
        self.snake.advance(snake_will_hit_food);

        if !snake_will_hit_food {
            return None;
        }

        let spawn_result = self.random_valid_food_position();
        if let Some(position) = spawn_result {
            self.food_position = position;
            None
        } else {
            // Failed to spawn food, can only happen when the snake fills the entire board.
            // So if we get here, the player has actually won.
            Some(SimulationResult::Won)
        }
    }

    /// Attempts to find a random valid location to put a new piece of snake
    /// food. Returns a [`Vector2`] representing the generated position if
    /// at least one free cell exists, otherwise a [`FoodSpawnError`] indicating
    /// the failure reason
    fn random_valid_food_position(&self) -> Option<Vector2> {
        self.board
            .random_free_cell(self.snake.len(), |cell| self.snake.contains(cell))
    }
}
