mod ext_x_i_frame_stream_inf;
mod ext_x_media;
mod ext_x_session_data;
mod ext_x_session_key;
mod ext_x_stream_inf;

pub use ext_x_i_frame_stream_inf::*;
pub use ext_x_media::*;
pub use ext_x_session_data::*;
pub use ext_x_session_key::*;
pub use ext_x_stream_inf::*;

#[derive(Debug)]
pub enum MasterPlaylistTags {
    ExtXMedia(ExtXMedia),
    ExtXStreamInf(ExtXStreamInf),
    ExtXIFrameStreamInf(ExtXIFrameStreamInf),
    ExtXSessionData(ExtXSessionData),
    ExtXSessionKey(ExtXSessionKey),
}
