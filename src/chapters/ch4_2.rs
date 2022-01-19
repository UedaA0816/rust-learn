
pub fn references_and_borrowing() {

  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);


  let mut s = String::from("hello");

  change(&mut s);

  println!("The value of '{}'.", s);


  let mut s = String::from("hello");

  let r1 = &mut s;
  let r2 = &mut s;

  //同時にsの参照を作成し利用しようとしているのでエラーになる
  // println!("{}, {}", r1, r2);


  let mut s = String::from("hello");

  {
    let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.

  let r2 = &mut s;


  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  // let r3 = &mut s; // BIG PROBLEM

  // println!("{}, {}, and {}", r1, r2, r3);


  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{}", r3);


  // let reference_to_nothing = dangle();
}

// 借用する際は型の前に&をつける
fn calculate_length(s: &String) -> usize {
  s.len()
}

// 借用した変数の値を変更させる場合
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// sはdangleが終了した時に解放されるがその参照を戻り値にしているのでエラー
// fn dangle() -> &String {
//   let s = String::from("hello");

//   &s
// }