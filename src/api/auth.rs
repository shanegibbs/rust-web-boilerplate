use rocket_contrib::JSON;
use validation::user::UserSerializer;

use diesel::prelude::*;
use diesel;

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use helpers::db::DB;


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: JSON<UserSerializer>, db: DB) -> String {
    let results = users.filter(email.eq(user_in.email.clone()))
        .first::<UserModel>(db.conn());

    if results.is_err() {
        return "404".to_string();
    }

    let user = results.unwrap();
    if !user.verify_password(user_in.password.as_str()) {
        return "no login".to_string();
    }

    user.generate_auth_token("loginsalt")
}

#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: JSON<UserSerializer>, db: DB) -> String {
    let results = users.filter(email.eq(user.email.clone()))
        .first::<UserModel>(db.conn());
    if results.is_ok() {
        return "conflict".to_string();
    }

    let new_password_hash = UserModel::make_password_hash(user.password.as_str());
    let new_user = NewUser {
        email: user.email.clone(),
        password_hash: new_password_hash,
    };

    diesel::insert(&new_user)
        .into(users::table)
        .execute(db.conn())
        .expect("Error saving new post");
    "lol".to_string()
}
