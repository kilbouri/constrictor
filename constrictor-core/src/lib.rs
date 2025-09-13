pub mod math {
    pub mod direction;
    pub mod vector2;

    pub use direction::*;
    pub use vector2::*;
}

pub mod models {
    pub mod snake;

    pub use snake::*;
}
