use uuid::Uuid;

pub struct Session {
    id: Uuid,
}

impl Session {
    pub fn new() -> Session {
        let id = Uuid::new_v4();

        Session { id }
    }
}

impl Session {
    /// Return Id String
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}
