extern crate advent_of_code;

use advent_of_code::day3;
use advent_of_code::input;

fn main() {
    let input = input::read_file_to_string("input/day3");

    // # Part 1
    let red_path: Vec<day3::Pathlet>;
    let green_path: Vec<day3::Pathlet>;
    let mut wires = input.lines();
    red_path = day3::string_to_path(wires.next().unwrap());
    green_path = day3::string_to_path(wires.next().unwrap());
    let red = day3::trace_wire(red_path);
    let green = day3::trace_wire(green_path);
    println!(
        "{}",
        day3::manhattan_distance(
            day3::Point::Origin(),
            *red.intersection(&green).next().unwrap()
        )
    );
}
