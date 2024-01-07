use chrono::{DateTime, NaiveDate, Utc};

pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (u32, u32, u32));
}

pub struct CustomData {
    pub random: u32,
}
