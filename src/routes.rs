use askama::Template;
use axum::extract::{Path, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::Html;
use crate::AppState;
use crate::models::{OnOff, Todo};
use crate::templates::{TodoListTemplate, TodoTemplate};

pub async fn show_tasks(
  State(state): State<AppState>,
) -> Html<String> {
  let todos = sqlx::query_as::<_, Todo>("SELECT * FROM TODOS")
    .fetch_all(&state.db)
    .await
    .unwrap();

  Html(TodoListTemplate { todos: &todos }.render().unwrap())
}

pub async fn add_todo(
  State(state): State<AppState>,
  Form(input): Form<AddTodo>
) -> Html<String> {
  let todo = sqlx::query_as::<_, Todo>(
    "INSERT INTO TODOS (title, description) VALUES ($1, $2) RETURNING id, title, description, completed",
  ) .bind(input.title)
    .bind(input.description)
    .fetch_one(&state.db)
    .await
    .unwrap();

  Html(TodoTemplate { todo: &todo }.render().unwrap())
}

#[derive(serde::Deserialize)]
pub struct AddTodo {
  pub title: String,
  pub description: String,
}

pub async fn complete_todo(
  Path(id): Path<i32>,
  State(state): State<AppState>,
  Form(input): Form<CompleteTodo>
) -> StatusCode {
  let _ = sqlx::query(
    "UPDATE TODOS SET completed = $1 WHERE id = $2",
  ) .bind(input.completed.and_then(|completed| Some(completed.is_on())).unwrap_or(false))
    .bind(id)
    .execute(&state.db)
    .await
    .unwrap();

  StatusCode::OK
}

#[derive(serde::Deserialize)]
pub struct CompleteTodo {
  pub completed: Option<OnOff>,
}