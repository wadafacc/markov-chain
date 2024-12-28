use std::{collections::HashMap, env::args, fs::read_to_string};

use rand::{thread_rng, Rng};

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

fn get_random_key<'a>(map: HashMap<&'a str, Vec<&'a str>>) -> &'a str {
  let keys = map.into_keys().collect();
  get_random_element(keys)
}

fn get_random_element(v: Vec<&str>) -> &str {
    let mut rand = thread_rng();

    v[rand.gen_range(0..v.len())]
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
