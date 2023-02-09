#[derive(Debug, Clone)]
pub struct ViewId {
    uuid: String,
}

impl ViewId {
    pub fn new(uuid: &str) -> Self {
        let uuid = uuid.to_owned();
        Self { uuid }
    }
}
