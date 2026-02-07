pub struct Account {
    pub id: u32,
    pub balance: f64,
}

impl Account {
    pub fn new(id: u32, balance: f64) -> Self {
        Self { id, balance }
    }
}
