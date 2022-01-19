
pub fn what_is_ownership() {
  //スタック
  // 後入れ先出し
  // 格納されるすべてのデータは、既知の固定サイズである必要がある

  //ヒープ
  // ヒープにデータを配置するときは、一定量のスペースを要求します。
  // メモリアロケータは、ヒープ内で十分な大きさの空のスポットを見つけ、それを使用中であるとマークし、その場所のアドレスであるポインタを返します。

  //データサイズが固定のためスタック(str)
  let s = "hello";

  //データサイズが可変の時はString
  let mut s = String::from("hello");

  s.push_str(", world!");

  println!("{}", s);


  let x = 5;
  let y = x;


  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s2);


  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("{}, world!", s1);


  let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
  let s2 = String::from("hello");     // s2 comes into scope

  let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

                                        
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);


} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}