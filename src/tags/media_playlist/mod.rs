mod ext_x_discontinuity_sequence;
mod ext_x_endlist;
mod ext_x_i_frames_only;
mod ext_x_media_sequence;
mod ext_x_playlist_type;
mod ext_x_targetduration;

pub use ext_x_discontinuity_sequence::*;
pub use ext_x_endlist::*;
pub use ext_x_i_frames_only::*;
pub use ext_x_media_sequence::*;
pub use ext_x_playlist_type::*;
pub use ext_x_targetduration::*;

use crate::types::MediaPlaylistType;

#[derive(Debug)]
pub enum MediaPlaylistTag {
    ExtXTargetduration(u64),
    ExtXMediaSequence(u64),
    ExtXDiscontinuitySequence(u64),
    ExtXEndlist,
    ExtXPlaylistType(MediaPlaylistType),
    ExtXIFramesOnly,
}
