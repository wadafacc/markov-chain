use std::{collections::HashMap, fmt::Error};
use crate::db::{database::Database, schema::Token};

pub struct Markov {
  current: Token
}


impl Markov {
  pub async fn get_jibberish(db: Database) -> Result<String, Error> {
    let start = db.fetch_one("SELECT * FROM tokens WHERE ORDER BY RANDOM() LIMIT 1").await?;



    todo!()
  }
  
  fn create_matrix(data: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for ln in data {
      let ln: Vec<&str> = ln.split(" ").collect();
      for i in 0..ln.len() {
        let c = ln[i]; // current word
        let t = if i + 1 >= ln.len() { "\n" } else { ln[i + 1] }; // next word
    
        if let Some(k) = map.get_mut(c) {
          k.push(t);
        } else {
          map.insert(c, vec![t]);
        }
      }
    }

    map
  }

}