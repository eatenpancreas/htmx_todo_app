use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub completed: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OnOff {
  On,
  Off
}

impl OnOff {
  pub(crate) fn is_on(&self) -> bool { match self { OnOff::On => true, OnOff::Off => false }}
}