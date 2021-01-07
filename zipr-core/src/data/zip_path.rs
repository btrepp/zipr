#[derive(Debug, PartialEq)]
pub struct ZipPath<'a>(&'a [u8]);

#[derive(Debug)]
pub enum ZipPathError {}

impl<'a> ZipPath<'a> {
    pub fn create_from_bytes(bytes: &'a [u8]) -> Result<Self, ZipPathError> {
        // need to validate this in future
        let path = ZipPath(bytes);
        Ok(path)
    }

    pub fn to_bytes(&self) -> &'a [u8] {
        self.0
    }
}
