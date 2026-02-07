pub struct User {
    pub id: u32,
    pub name: String,
    pub accounts: Vec<u32>,      // List of account IDs the user owns
}
