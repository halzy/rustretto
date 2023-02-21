#[derive(Debug, Clone)]
pub struct ViewId {
    uuid: String,
}

impl std::fmt::Display for ViewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.uuid)
    }
}

impl AsRef<str> for ViewId {
    fn as_ref(&self) -> &str {
        &self.uuid
    }
}

impl ViewId {
    pub fn new(uuid: &str) -> Self {
        let uuid = uuid.to_owned();
        Self { uuid }
    }
}
