

pub fn if_let() {

  let some_u8_value = Some(3);
  if let Some(3) = some_u8_value {
    println!("three");
  }


  // let mut count = 0;
  // if let Coin::Quarter(state) = coin {
  //     println!("State quarter from {:?}!", state);
  // } else {
  //     count += 1;
  // }
}