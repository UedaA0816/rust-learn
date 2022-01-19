
pub fn slices() {
  let mut text = String::from("hello_world");
  let first = first_word(&text);
  text.clear();
  //ただのインデックスとして戻り値にすると、元の文字列が変更された時もインデックスはそのままとなる
  println!("{},{}",text,first);


  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];


  let mut text2 = String::from("hello_world");
  let first = slice_first_word(&text2);
  // 文字列の参照を返すことによって、その文字列の参照が解放されるまでは、元の文字列が変わらないことをコンパイル時点で保証する
  //エラーになる
  // text2.clear();
  println!("{},{}",text,first);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return i;
      }
  }

  s.len()
}

fn slice_first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}