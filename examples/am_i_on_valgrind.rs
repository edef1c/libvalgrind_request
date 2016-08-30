extern crate valgrind_request;

fn main() {
  println!("currently {} Valgrinds deep", valgrind_request::running_on_valgrind());
}
