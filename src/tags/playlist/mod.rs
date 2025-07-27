use crate::tags::playlist::ext_x_start::ExtXStart;

pub(crate) mod ext_x_independent_segments;
pub(crate) mod ext_x_start;

#[derive(Debug)]
pub(crate) enum PlaylistTags {
    ExtXIndependentSegments,
    ExtXStart(ExtXStart),
}
