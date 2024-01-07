pub struct User {
    id: uuid::Uuid,
    name: String,
    birth_date: NaiveDate,
    custom_data: CustomData,
}

pub struct CustomData {
    random: u32,
}
