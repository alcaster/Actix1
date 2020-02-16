use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, PartialEq, Associations)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct Game {
    pub id: i32,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}