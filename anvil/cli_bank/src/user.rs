pub struct User {
    pub id: u32,
    pub name: String,
}

pub struct UserGenerator {
    next_id: u32,
}

impl UserGenerator {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn create_user(&mut self, name: String) -> User {
        let id = self.next_id;
        self.next_id += 1;
        User { id, name }
    }
}

