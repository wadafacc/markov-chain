use std::{collections::HashMap, env::args, fs::read_to_string};

use rand::{thread_rng, Rng};

fn main() {
  let input = args().nth(1).expect("need input string");
  let word_count = args()
    .nth(2)
    .unwrap_or("1".to_string())
    .parse::<i32>()
    .unwrap();

  let str = read_to_string("./data")
    .expect("no data found")
    .replace(".", " ")
    .replace(",", " ")
    .replace("-", " ")
    .replace("\n", " ")
    .to_lowercase();

  let matrix = create_matrix(str.split(" ").collect());
  let mut out: Vec<&str> = Vec::new();

  let mut current_token = input.split(" ").last().unwrap();
  for _ in 0..word_count {
    if let Some(v) = matrix.get(current_token) {
      // println!("{:?}", v);
      let t = get_token(v.to_vec());
      out.push(t);
      current_token = t;
    }
  }

  println!("{:?}", out);
}

fn get_token(v: Vec<&str>) -> &str {
    let mut rand = thread_rng();

    v[rand.gen_range(0..v.len())]
}

fn create_matrix(data: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for i in 0..(data.len() - 1) {
        let c = data[i]; // current word
        let t = data[i + 1]; // next word

        if let Some(k) = map.get_mut(c) {
            k.push(t);
        } else {
            map.insert(c, vec![t]);
        }
    }

    map
}
