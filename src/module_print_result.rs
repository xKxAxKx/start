pub fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
  match ans {
      Ok(res) => println!("{}", res),
      Err(str) => println!("{}", str),
  }
}