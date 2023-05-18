use ::utils::file::read_file;

mod calculate_final_floor;

use calculate_final_floor::calculate_final_floor;

pub fn driver() -> i32 {
  let floors: String = read_file("day_one.txt");

  calculate_final_floor(floors)
}