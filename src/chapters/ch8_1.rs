
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn vectors() {
  
  let mut v: Vec<i32> = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let v = vec![1, 2, 3];


  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  let third: Option<&i32> = v.get(2);


  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];

  let third = row.get(3);

  if let Some(SpreadsheetCell) = third {
    println!("some")
  }

}