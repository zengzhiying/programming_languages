use std::{collections::HashMap, sync::{atomic::{AtomicUsize, Ordering}, Mutex}};
use axum::{extract::{Path, Query}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use lazy_static::*;

static USER_ID_GENERATE: AtomicUsize = AtomicUsize::new(1);

lazy_static! {
    static ref USER_MAP: Mutex<HashMap<u64, String>> = {
        let user_map: HashMap<u64, String> = HashMap::new();
        Mutex::new(user_map)
    };
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // 11k/s CPU: 150
        .route("/hello", get(hello))
        // 10s/s CPU: 160
        .route("/hello-calc", get(hello_calc))
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/query", get(query_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "hello world"
}

async fn hello_calc() -> (StatusCode, Json<HashMap<i32, String>>) {
    let mut m = HashMap::new();
    for i in 0..10 {
        m.insert(i, String::from("hello"));
    }
    (StatusCode::OK, Json(m))
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: USER_ID_GENERATE.fetch_add(1, Ordering::Relaxed) as u64,
        username: payload.username
    };
    USER_MAP.lock().unwrap().insert(user.id, user.username.clone());
    (StatusCode::CREATED, Json(user))
}

async fn get_user(Path(user_id): Path<u64>) -> (StatusCode, String) {
    if user_id == 0 {
        return (StatusCode::NOT_FOUND, "".to_string());
    }

    let data = USER_MAP.lock().unwrap();
    if let Some(user) = data.get(&user_id) {
        (StatusCode::OK, user.to_string())
    } else {
        (StatusCode::NOT_FOUND, "".to_string())
    }
}

async fn query_user(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<User>) {
    let q = params.get("user");
    if let Some(user) = q {
        if user.as_str() == "" {
            return (StatusCode::NOT_FOUND, Json(User{id: 0, username: "".to_string()}));
        }
        let data = USER_MAP.lock().unwrap();
        for (user_id, username) in data.iter() {
            if username == user {
                return (StatusCode::OK, Json(User{id: *user_id, username: username.to_string()}));
            }
        }
        return (StatusCode::NOT_FOUND, Json(User{id: 0, username: "".to_string()}));
    }
    (StatusCode::NOT_FOUND, Json(User{id: 0, username: "".to_string()}))
}
