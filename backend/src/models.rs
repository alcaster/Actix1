use serde::{Serialize, Deserialize};

use crate::schema::users;
use crate::schema::games;
use crate::schema::user_games;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, PartialEq, Associations)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug, Clone, Identifiable, Queryable, Associations, AsChangeset, Deserialize)]
pub struct Game {
    pub id: i32,
    pub start: NaiveDateTime,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub start: NaiveDateTime,
    pub active: bool,
}


#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable, PartialEq, Associations)]
#[belongs_to(User)]
#[belongs_to(Game)]
pub struct UserGame {
    pub id: i32,
    pub user_id: String,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name="user_games"]
pub struct NewUserGame {
    pub user_id: String,
    pub game_id: i32,
}

