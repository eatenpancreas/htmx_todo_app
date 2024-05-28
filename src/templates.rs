use askama::Template;
use crate::models::Todo;

#[derive(Template)]
#[template(path="todo_list.html")]
pub struct TodoListTemplate<'a> {
  pub(crate) todos: &'a Vec<Todo>,
}

#[derive(Template)]
#[template(path="todo.html")]
pub struct TodoTemplate<'a> {
  pub(crate) todo: &'a Todo,
}
