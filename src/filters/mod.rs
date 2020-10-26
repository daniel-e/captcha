//! Filters to disturb and transform CAPTCHAs.

mod cow;
mod dots;
mod grid;
mod noise;
mod wave;

use images::Image;

// reexports
pub use filters::cow::Cow;
pub use filters::dots::Dots;
pub use filters::grid::Grid;
pub use filters::noise::Noise;
pub use filters::wave::Wave;

pub trait Filter {
    fn apply(&self, i: &mut Image);
}
