use crate::version::Version;

pub(crate) mod ext_x_version;
pub(crate) mod extm3u;

#[derive(Debug)]
pub(crate) enum BasicTags {
    Extm3u,
    ExtXVersion(Version),
}
