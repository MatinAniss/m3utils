use crate::tags::master_playlist::{
    ext_x_i_frame_stream_inf::ExtXIFrameStreamInf, ext_x_media::ExtXMedia,
    ext_x_session_data::ExtXSessionData, ext_x_session_key::ExtXSessionKey,
    ext_x_stream_inf::ExtXStreamInf,
};

pub(crate) mod ext_x_i_frame_stream_inf;
pub(crate) mod ext_x_media;
pub(crate) mod ext_x_session_data;
pub(crate) mod ext_x_session_key;
pub(crate) mod ext_x_stream_inf;

#[derive(Debug)]
pub(crate) enum MasterPlaylistTags {
    ExtXMedia(ExtXMedia),
    ExtXStreamInf(ExtXStreamInf),
    ExtXIFrameStreamInf(ExtXIFrameStreamInf),
    ExtXSessionData(ExtXSessionData),
    ExtXSessionKey(ExtXSessionKey),
}
