use crate::types::EncryptionMethod;

#[derive(Debug)]
pub struct SessionKey {
    pub method: EncryptionMethod,
    pub uri: String,
    pub iv: Option<u128>,
    pub key_format: Option<String>,
    pub key_format_versions: Option<String>,
}
