extern crate rust_buzz;

#[test]
fn says_fizz() {
  assert_eq!("fizz", rust_buzz::say(3));
}

#[test]
fn says_buzz() {
  assert_eq!("buzz", rust_buzz::say(5));
}

#[test]
fn says_fizzbuzz() {
  assert_eq!("fizzbuzz", rust_buzz::say(15));
}

#[test]
fn returns_number() {
  assert_eq!("1", rust_buzz::say(1));
}
