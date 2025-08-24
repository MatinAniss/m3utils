use crate::types::Version;

mod ext_x_version;
mod extm3u;

pub use ext_x_version::*;
pub use extm3u::*;

#[derive(Debug)]
pub enum BasicTag {
    Extm3u,
    ExtXVersion(Version),
}
