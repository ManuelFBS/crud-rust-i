use crate::user::User;

pub trait Repository {
    fn get_users(&self, user_id: uuid::Uuid) -> Result<User, String>;
}

pub struct MemoryRepository {
    users: Vec<User>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new("Manuel".to_string(), (1962, 03, 04))],
        }
    }
}
