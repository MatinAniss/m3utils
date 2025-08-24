mod ext_x_byterange;
mod ext_x_daterange;
mod ext_x_discontinuity;
mod ext_x_key;
mod ext_x_map;
mod ext_x_program_date_time;
mod extinf;

pub use ext_x_byterange::*;
pub use ext_x_daterange::*;
pub use ext_x_discontinuity::*;
pub use ext_x_key::*;
pub use ext_x_map::*;
pub use ext_x_program_date_time::*;
pub use extinf::*;

use crate::types::{ByteRange, DateRange, DateTime, Key, Map};

#[derive(Debug)]
pub enum MediaSegmentTag {
    Extinf(Extinf),
    ExtXByterange(ByteRange),
    ExtXDiscontinuity,
    ExtXKey(Key),
    ExtXMap(Map),
    ExtXProgramDateTime(DateTime),
    ExtXDaterange(DateRange),
}
