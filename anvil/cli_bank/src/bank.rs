use std::collections::HashMap;

use crate::user::User;
use crate::account::Account;
use crate::new::{register_new_account};

pub struct Bank {
    pub accounts: HashMap<u32, Account>,
    pub users: HashMap<u32, User>,
    pub next_account_id: u32,
    pub next_user_id: u32,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            users: HashMap::new(),
            next_account_id: 0,
            next_user_id: 0,
        }
    }

    pub fn generate_next_user_id(&mut self) -> u32 {
        let id = self.next_user_id;
        self.next_user_id += 1;
        id
    }

    pub fn generate_next_account_id(&mut self) -> u32 {
        let id = self.next_account_id;
        self.next_account_id += 1;
        id
    }

    pub fn add_account(&mut self, initial_balance: f64) -> u32 {
        let id = self.generate_next_account_id();
        let account = register_new_account(id, initial_balance);

        self.accounts.insert(id, account);

        id
    }

    pub fn deposit(&mut self, account_id: u32, amount: f64) -> Result<(), String> {
        match self.accounts.get_mut(&account_id) {
            Some(account) => {
               account.balance += amount;
                Ok(())
            }
            None => Err(format!("Account {} not found", account_id)),
        }
    }

    pub fn withdraw(&mut self, account_id: u32, amount: f64) -> Result<(), String> {
        match self.accounts.get_mut(&account_id) {
            Some(account) => {
                if account.balance >= amount {
                    account.balance -= amount;
                    Ok(())
                } else {
                    Err("Insufficient funds".to_string())
                }
            }
            None => Err(format!("Account {} not found", account_id)),
        }
    }

    pub fn get_balance(&self, account_id: u32) -> Option<f64> {
        self.accounts.get(&account_id).map(|a| a.balance)
    }
}
