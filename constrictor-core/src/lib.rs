pub mod math {
    pub mod direction;
    pub mod vector2;

    pub use direction::*;
    pub use vector2::*;
}

pub mod models {
    pub mod board;
    pub mod snake;
    pub mod snake_simulation;

    pub use board::*;
    pub use snake::*;
    pub use snake_simulation::*;
}
