mod chapters;

fn main() {
  println!("main");
  chapters::ch2_1::guessing_game();
  chapters::ch3_1::var_and_mut();
  chapters::ch3_2::data_types();
  chapters::ch3_3::functions();
  chapters::ch3_4::comments();
  chapters::ch3_5::control_flow();
  chapters::ch4_1::what_is_ownership();
  chapters::ch4_2::references_and_borrowing();
  chapters::ch4_3::slices();
  chapters::ch5_1::defining_structs();
  chapters::ch5_2::example_structs();
  chapters::ch5_3::method_syntax();
  chapters::ch6_1::defining_an_enum();
  chapters::ch6_2::match_operator();
  chapters::ch6_3::if_let();
  chapters::ch8_1::vectors();
  chapters::ch8_2::strings();
}