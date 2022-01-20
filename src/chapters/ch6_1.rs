
enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}


enum IpAddr2 {
  V4(String),
  V6(String),
}

enum IpAddr3 {
  V4(u8, u8, u8, u8),
  V6(String),
}


enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
      // method body would be defined here
      // メソッド本体はここに定義される
  }
}



pub fn defining_an_enum() {


  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  
  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  };


  let home = IpAddr2::V4(String::from("127.0.0.1"));

  let loopback = IpAddr2::V6(String::from("::1"));


  let home = IpAddr3::V4(127, 0, 0, 1);

  let loopback = IpAddr3::V6(String::from("::1"));


  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
  
}