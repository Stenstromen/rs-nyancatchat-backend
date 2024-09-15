// src/chat_model.rs

use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::db_mysql::get_connection;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomUser {
    pub room: String,
    pub user: String,
}

pub struct AppState {
    pub room_users: Mutex<Vec<RoomUser>>,
    pub pool: Pool,
}

impl AppState {
    pub fn add_user(&self, room: String, user: String) {
        let mut room_users = self.room_users.lock().unwrap();
        room_users.push(RoomUser { room, user });
    }

    pub fn remove_user(&self, room: &str, user: &str) {
        let mut room_users = self.room_users.lock().unwrap();
        if let Some(pos) = room_users.iter().position(|u| u.room == room && u.user == user) {
            room_users.remove(pos);
        }
    }

    pub fn insert_message(
        &self,
        user: &str,
        message_content: &str,
        message_iv: &str,
        room: &str,
    ) {
        let mut conn = get_connection(&self.pool).unwrap();
        conn.exec_drop(
            "INSERT INTO msgtable (user, messagecontent, messageiv, room) VALUES (:user, :messagecontent, :messageiv, :room)",
            params! {
                "user" => user,
                "messagecontent" => message_content,
                "messageiv" => message_iv,
                "room" => room,
            },
        )
        .unwrap();
        println!("Message from {} inserted into the database", user);
    }

    pub fn check_for_messages_user(&self, user: &str) -> usize {
        let mut conn = get_connection(&self.pool).unwrap();
        let count: u64 = conn
            .exec_first(
                "SELECT COUNT(*) FROM msgtable WHERE user = :user",
                params! {
                    "user" => user,
                },
            )
            .unwrap()
            .unwrap_or(0);
        count as usize
    }

    pub fn delete_messages(&self, user: &str) {
        let mut conn = get_connection(&self.pool).unwrap();
        conn.exec_drop(
            "DELETE FROM msgtable WHERE user = :user",
            params! {
                "user" => user,
            },
        )
        .unwrap();
        println!("Deleted messages for user {}", user);
    }
}