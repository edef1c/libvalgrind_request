extern crate valgrind;

fn main() {
  println!("currently {} Valgrinds deep", valgrind::running_on_valgrind());
}
