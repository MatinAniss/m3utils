mod ext_x_independent_segments;
mod ext_x_start;

pub use ext_x_independent_segments::*;
pub use ext_x_start::*;

use crate::types::Start;

#[derive(Debug)]
pub enum PlaylistTag {
    ExtXIndependentSegments,
    ExtXStart(Start),
}
