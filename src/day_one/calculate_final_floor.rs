pub fn calculate_final_floor(floors: String) -> i32 {
  let mut up = 0;
  let mut down = 0;

  for character in floors.chars() {
    if character == '(' {
      up += 1;
    } else {
      down += 1;
    }
  }

  up - down
}

#[cfg(test)]
mod calculate_final_floor_tests {
    use super::calculate_final_floor;

  #[test]
  fn final_floor_test() {
    let final_floors_equal: [i32; 5] = [
      calculate_final_floor(String::from("()()")),
      calculate_final_floor(String::from("(((")),
      calculate_final_floor(String::from("(()(()(")),
      calculate_final_floor(String::from(")))")),
      calculate_final_floor(String::from(")())())")),
    ];

    assert_eq!(final_floors_equal, 
      [0, 3, 3, -3, -3], 
      "floors did not evaluate to 0"
    )
  }
}