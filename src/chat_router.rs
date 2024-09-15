// src/chat_router.rs

use std::env;
use mysql::prelude::*;
use mysql::params;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::chat_model::{self, AppState};
use crate::crypto_enc;
use crate::db_mysql::get_connection;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_messages)
        .service(web::resource("/socket.io/").to(crate::socket_handlers::handle_socket));
}

#[derive(Serialize)]
struct User {
    room: String,
    user: String,
}

#[get("/getusers/{room}")]
async fn get_users(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let room = path.into_inner();
    let room_users = data.room_users.lock().unwrap();
    let users_in_room: Vec<User> = room_users
        .iter()
        .filter(|u| u.room == room)
        .map(|u| User {
            room: u.room.clone(),
            user: u.user.clone(),
        })
        .collect();
    HttpResponse::Ok().json(users_in_room)
}

#[derive(Serialize)]
struct Message {
    origin: String,
    user: String,
    message: String,
}

#[get("/getmessages/{room}/{user}")]
async fn get_messages(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (room, user) = path.into_inner();
    let mut conn = get_connection(&data.pool).unwrap();

    let rows: Vec<(String, String, String, String)> = conn
        .exec(
            "SELECT user, messagecontent, messageiv, room FROM msgtable WHERE room = :room",
            params! {
                "room" => &room,
            },
        )
        .unwrap();

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let messages: Vec<Message> = rows
        .into_iter()
        .map(|(db_user, content, iv, _)| {
            let decrypted_message = crypto_enc::decrypt(
                &secret_key,
                &iv,
                &content,
            );
            Message {
                origin: if db_user == user {
                    "sender".to_string()
                } else {
                    "server".to_string()
                },
                user: db_user,
                message: decrypted_message,
            }
        })
        .collect();

    HttpResponse::Ok().json(messages)
}