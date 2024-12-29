use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Token {
  pub key: String,
  pub value: String,
  pub guild: String,
  pub leading: bool
}