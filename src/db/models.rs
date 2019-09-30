use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    // password: String,
    pub first_name: String,
    pub last_name: String,
    // pub yaily_id: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    // pub yapily_id &'a str,
}
