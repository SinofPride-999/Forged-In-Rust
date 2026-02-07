use crate::account::Account;
use crate::user::User;
use crate::bank::Bank;

pub fn register_new_user(id: u32, name: String) -> User {
    let user = User::new(id, name);
    user
}

pub fn register_new_account(id: u32, balance: f64) -> Account{
    let account = Account::new(id, balance);
    account
}

pub fn register_new_bank() -> Bank {
    let bank = Bank::new();
    bank
}
