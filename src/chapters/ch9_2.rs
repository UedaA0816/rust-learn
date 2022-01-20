use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


pub fn recoverable_errors_with_result() {
  
  // let f = File::open("hello.txt");

  // let f = match f {
  //   Ok(file) => file,
  //   // Err(ref error) if error.kind() == ErrorKind::NotFound => {
  //   //     match File::create("hello.txt") {
  //   //         Ok(fc) => fc,
  //   //         Err(e) => {
  //   //             panic!(
  //   //                 //ファイルを作成しようとしましたが、問題がありました
  //   //                 "Tried to create file but there was a problem: {:?}",
  //   //                 e
  //   //             )
  //   //         },
  //   //     }
  //   // },
  //   Err(error) => {
  //       panic!(
  //           "There was a problem opening the file: {:?}",
  //           error
  //       )
  //   },
  // };


  // let f = File::open("hello.txt").unwrap();


  // hello.txtを開くのに失敗しました
  // let f = File::open("hello.txt").expect("Failed to open hello.txt");


  let name1 = read_username_from_file().unwrap();
  let name2 = read_username_from_file_question_operator().unwrap();
  let name3 = read_username_from_file_question_operator_more().unwrap();

  println!("user:{},{},{}",name1,name2,name3)

}


fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
  }
}

fn read_username_from_file_question_operator() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file_question_operator_more() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}