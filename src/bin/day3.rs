extern crate advent_of_code;

use advent_of_code::day3;
use advent_of_code::input;

fn main() {
    let input = input::read_file_to_string("input/day3");

    let mut wires = input.lines();
    let red_path = day3::string_to_path(wires.next().unwrap());
    let green_path = day3::string_to_path(wires.next().unwrap());
    let red = day3::trace_wire(red_path);
    let green = day3::trace_wire(green_path);
    let intersections = day3::wire_intersections(&red, &green);

    // # Part 1
    let closest = intersections.iter().map(|x| x.distance()).min().unwrap();
    println!("{}", closest);

    // # Part 2
    let shortest = intersections
        .iter()
        .map(|x| day3::wire_length_to(&red, x) + day3::wire_length_to(&green, x))
        .min()
        .unwrap();
    println!("{}", shortest);
}
