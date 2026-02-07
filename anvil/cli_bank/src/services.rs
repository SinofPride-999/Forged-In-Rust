use crate::user::UserGenerator;

use super::user::User;

pub fn create_user(name: String) -> User {
    let mut generator = UserGenerator::new();

    let user = generator.create_user(name);

    user
}
