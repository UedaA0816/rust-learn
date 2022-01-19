
pub fn functions() {
  
  println!("Hello, world!");

  another_function(5, 'h');


  let y = 6;

  let y = {
    let x = 3;
    x + 1 //最後にセミコロンがないと式として評価されyに計算結果が代入される
  };

  println!("The value of y is: {}", y);


  let x = five();

  println!("The value of x is: {}", x);


  let x = plus_one(5);

  println!("The value of x is: {}", x);
}

fn another_function(value: i32,unit_label: char) {
  println!("The measurement is: {}{}", value, unit_label);

}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1 //セミコロンをつけると戻り値がなくなる
}