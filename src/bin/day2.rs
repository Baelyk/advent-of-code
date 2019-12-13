extern crate advent_of_code;

use advent_of_code::day2;
use advent_of_code::input;

fn main() {
    let input = input::read_file_to_string("input/day2part2");

    // # Part 1
    let mut program = day2::string_to_intcode(&input);
    program[1] = 12;
    program[2] = 2;
    day2::intcode_execute(&mut program);
    println!("{}", program[0]);

    // # Part 2
    for noun in 0..=99 {
        for verb in 0..=99 {
            program = day2::string_to_intcode(&input);
            program[1] = noun;
            program[2] = verb;
            day2::intcode_execute(&mut program);
            if program[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break;
            }
        }
        if program[0] == 19690720 {
            break;
        }
    }
}
