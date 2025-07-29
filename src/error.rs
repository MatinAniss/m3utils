#[derive(Debug)]
pub enum Error {
    /// This error denotes that the EXTM3U tag is missing from the begining of the playlist.
    MissingExtm3u,

    /// This error denotes that there is a duplicate tag that is not allowed.
    DuplicateTag,

    /// This error denotes that a tag's configuration is invalid.
    InvalidTag,

    /// This error denotes that a tag is missing required data.
    IncompleteTag,

    /// This error denotes that a tag is incompatible with the playlist type.
    IncompatibleTag,
}
