pub trait Repository {
    fn get_users(&self, user_id: uuid::Uuid) -> Result<User, String>;
}

pub struct MemoryRepository {
    user: Vec<User>,
}
