#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub additional_id: Option<i64>,
    pub access_token: Option<String>,
}
