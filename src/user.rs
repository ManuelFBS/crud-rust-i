use chrono::{DateTime, NaiveDate, Utc};

pub struct User {
    id: uuid::Uuid,
    name: String,
    birth_date: NaiveDate,
    custom_data: CustomData,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

pub struct CustomData {
    random: u32,
}
