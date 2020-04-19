use diesel::prelude::*;

extern crate diesel;

use uuid::Uuid;
use chrono::Utc;
use crate::database::models;

/// Run query using Diesel to get all users.
pub fn get_all_users(
    conn: &PgConnection,
) -> Result<Vec<models::User>, diesel::result::Error> {
    use crate::database::schema::users::dsl::*;

    let all_users = users
        .load::<models::User>(conn);

    all_users
}

///// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::database::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

//
///// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::database::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;
    let a = vec![new_user.id.clone()];
    insert_new_game(a, &conn)?;
    Ok(new_user)
}

pub fn insert_new_game(
    // prevent collision with `name` column imported inside the function
    names: Vec<String>,
    conn: &PgConnection,
) -> Result<models::Game, diesel::result::Error> {
    use crate::database::schema::games::dsl::*;
    use crate::database::schema::user_games::dsl::*;

    let new_game = models::NewGame {
        start: Utc::now().naive_utc(),
        active: false,
    };
    let game: models::Game = diesel::insert_into(games).values(&new_game).get_result(conn).expect("Error saving new game");

    for i in &names {
        let new_user_game = models::NewUserGame{
            user_id: i.clone(),
            game_id: game.id
        };
        diesel::insert_into(user_games).values(&new_user_game).execute(conn).expect("Error saving new user_game");
    }
    Ok(game)
}