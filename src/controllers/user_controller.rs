use diesel::{PgConnection, prelude::*};
use actix_web::{web, HttpResponse};

use crate::models::{User, Pool};
use crate::utils::{request_structs::UserData, response_structs::GeneralResponse, password_hash::*};

pub async fn create_user(user_data: web::Json<UserData>, pool: web::Data<Pool>) -> Result<HttpResponse, HttpResponse> {
    let result = web::block(move || diesel_query(user_data.into_inner(), pool)).await;
    println!("result: {:?} ", result);

    result
        .map(|_| {
            let response = GeneralResponse {status: true, message: String::from("User created.")};
            HttpResponse::Ok().json(response)
        })
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

fn diesel_query(user_data: UserData, pool: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::users;

    let password: String = hash_password(&user_data.password).unwrap();
    let new_user = User::from_payload(user_data.user_id, user_data.name, password, user_data.email);

    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_user = diesel::insert_into(users).values(&new_user).get_result(conn).unwrap();

    dbg!(&inserted_user);

    Ok(inserted_user)
}