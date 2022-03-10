use diesel::Queryable;
use super::schema::notes;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub kind: i16,
    pub user: i64,
    pub issuer: i64,
    pub info: String,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote {
    pub kind: i16,
    pub user: i64,
    pub issuer: i64,
    pub info: String,
}
