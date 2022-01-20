
pub fn unrecoverable_errors_with_panic() {
  
  // panic!("crash and burn");  //クラッシュして炎上


  let v = vec![1, 2, 3];

  v[99];


  //環境変数”RUST_BACKTRACE”を0以外に設定するとバックトレースがログに表示される

}