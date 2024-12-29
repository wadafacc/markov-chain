use sqlx::{query, query_as, sqlite::SqliteQueryResult, Error, SqlitePool};

use super::schema::Token;

pub struct Database(SqlitePool);


impl Database {
  async fn new() -> Result<Self, Error> {
    Ok(Database(SqlitePool::connect("sqlite:data.db").await?))
  }

  pub async fn fetch(&self, query: &str) -> Result<Vec<Token>, Error> {
    Ok(query_as(query).fetch_all(&self.0).await?)
  }

  pub async fn fetch_one(&self, query: &str) -> Result<Token, Error> {
    Ok(query_as(query).fetch_one(&self.0).await?)
  }


  pub async fn insert(&self, t: Token) -> Result<SqliteQueryResult, Error> {
    let res = query("INSERT INTO tokens (key, value, guild) VALUES (?,?,?)")
    .bind(t.key)
    .bind(t.value)
    .bind(t.guild).execute(&self.0).await?;

    Ok(res)
  }
}