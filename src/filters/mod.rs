//! Filters to disturb and transform CAPTCHAs.

mod cow;
mod dots;
mod grid;
mod noise;
mod wave;

use crate::images::Image;

// reexports
pub use crate::filters::cow::Cow;
pub use crate::filters::dots::Dots;
pub use crate::filters::grid::Grid;
pub use crate::filters::noise::Noise;
pub use crate::filters::wave::Wave;

pub trait Filter {
    fn apply(&self, i: &mut Image);
}
