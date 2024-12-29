use std::{collections::HashMap, fs::read_to_string};

use rand::{thread_rng, Rng};

mod markov;
mod db;

fn main() {
  let str_in = read_to_string("./data_lorem").expect("no data found");

  let matrix = create_matrix(str_in.lines().collect());
  // matrix.iter().for_each(|(k,v)| println!("{} -> {:?}", k,v));

  let mut out: Vec<&str> = Vec::new();
  let mut current_token = get_random_key(matrix.clone());
  while current_token != "\n" {
    if let Some(v) = matrix.get(current_token) {
      // println!("{:?}", v);
      let t = get_random_element(v.to_vec());
      out.push(t);
      current_token = t;
    }
  }
  
  println!("{:?}", out.join(" "));
}
