
use std::collections::HashMap;


pub fn hash_maps() {
  
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);


  let teams  = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  //いろんなデータ構造にまとめ上げることができ、 コンパイラは指定しない限り、どれを所望なのかわからないからです。ところが、キーと値の型引数については、 アンダースコアを使用しており、コンパイラはベクタのデータ型に基づいてハッシュマップが含む型を推論することができるのです。
  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value);
  // field_name and field_value are invalid at this point, try using them and
  // see what compiler error you get!
  // field_nameとfield_valueはこの時点で無効になる。試しに使ってみて
  // どんなコンパイルエラーが出るか確認してみて！
  // println!("{}",field_name)


  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }


  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);


  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }

  println!("{:?}", map);
}