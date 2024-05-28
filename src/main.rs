mod models;
mod templates;
mod routes;

use std::sync::{Arc, Mutex};
use askama::Template;
use axum::response::{Html};
use axum::{Extension, Form, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::routes::{add_todo, complete_todo, show_tasks};

#[derive(Clone)]
struct AppState {
  db: PgPool
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres] db: PgPool
) -> shuttle_axum::ShuttleAxum {
  sqlx::migrate!()
    .run(&db)
    .await
    .expect("Looks like something went wrong with migrations :(");
  
  let router = Router::new()
    .route("/", get(show_tasks))
    .route("/todos", post(add_todo))
    .route("/todos/:id/complete", post(complete_todo))
    .with_state(AppState { db: db.clone() })
    .layer(Extension(Arc::new(db)));

  Ok(router.into())
}

