fn divisible_by_three(number: i32) -> bool {
  divisible_by(number, 3)
}

fn divisible_by_five(number: i32) -> bool {
  divisible_by(number, 5)
}

fn divisible_by_fifteen(number: i32) -> bool {
  divisible_by(number, 15)
}

fn divisible_by(number: i32, divisor: i32) -> bool {
  number % divisor == 0
}

pub fn say(number: i32) -> String {
  let result = if divisible_by_fifteen(number) {
    "fizzbuzz".to_string()
  } else if divisible_by_five(number) {
    "buzz".to_string()
  } else if divisible_by_three(number) {
    "fizz".to_string()
  } else {
    number.to_string()
  };
  result
}
