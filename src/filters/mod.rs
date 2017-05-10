pub mod noise;
mod grid;
mod cow;
mod dots;
mod wave;

use images::Image;

// reexports
pub use filters::noise::Noise;
pub use filters::grid::Grid;
pub use filters::cow::Cow;
pub use filters::dots::Dots;
pub use filters::wave::{Wave, Direction};

pub trait Filter {
    fn apply(&self, &mut Image);
}