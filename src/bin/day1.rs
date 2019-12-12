extern crate advent_of_code;

use advent_of_code::day1;
use advent_of_code::input;
use std::str::FromStr;

fn main() {
    let input = input::read_file_to_string("input/day1part1");

    // Part 1
    let mut fuel = 0;
    for line in input.lines() {
        let mass: usize = usize::from_str(line).unwrap();
        fuel += day1::fuel_from_mass(mass);
    }
    println!("Fuel: {}", fuel);

    // Part 2
    let mut fuel_fuel = 0;
    for line in input.lines() {
        let mass: usize = usize::from_str(line).unwrap();
        fuel_fuel += day1::fuel_for_module(mass);
    }
    println!("Fuel fuel: {}", fuel_fuel);
}
