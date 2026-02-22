use std::collections::HashMap;

use crate::user::User;
use crate::account::Account;
use crate::new::{register_new_account};

pub struct Bank {
    pub accounts: HashMap<u32, Account>,
    pub users: HashMap<u32, User>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, account_id: u32, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be greater than 0".to_string());
        }

        match self.accounts.get_mut(&account_id) {
            Some(account) => {
                account.balance += amount;
                println!("{} has been deposited successfully", amount);
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
