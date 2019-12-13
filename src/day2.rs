// Day 2 2019

#[cfg(test)]
mod tests {
    use crate::day2::*;
    #[test]
    fn test_string_to_intcode() {
        assert_eq!(string_to_intcode("1, 0,0,3,99"), vec![1, 0, 0, 3, 99]);
        assert_eq!(
            string_to_intcode("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
    #[test]
    fn test_intcode_add() {
        let program = &mut string_to_intcode("1, 1, 2, 0");
        intcode_add(program, 1, 2, 3);
        assert_eq!(program, &vec![3, 1, 2, 0]);
    }
    #[test]
    fn test_intcode_multiply() {
        let program = &mut string_to_intcode("2, 1, 2, 0");
        intcode_multiply(program, 1, 2, 3);
        assert_eq!(program, &vec![2, 1, 2, 0]);
    }
    #[test]
    fn test_intcode_execute() {
        let mut program = string_to_intcode("1,1,1,4,99,5,6,0,99");
        program = intcode_execute(&mut program).to_vec();
        assert_eq!(program, string_to_intcode("30,1,1,4,2,5,6,0,99"));

        let mut program1 = string_to_intcode("2,4,4,5,99,0");
        program1 = intcode_execute(&mut program1).to_vec();
        assert_eq!(program1, string_to_intcode("2,4,4,5,99,9801"));
    }
}

fn intcode_add(program: &mut Vec<usize>, first: usize, second: usize, third: usize) {
    let first_index = program[first];
    let second_index = program[second];
    let output_index = program[third];
    program[output_index] = program[first_index] + program[second_index];
}

fn intcode_multiply(program: &mut Vec<usize>, first: usize, second: usize, third: usize) {
    let first_index = program[first];
    let second_index = program[second];
    let output_index = program[third];
    program[output_index] = program[first_index] * program[second_index];
}

pub fn intcode_execute(program: &mut Vec<usize>) -> &Vec<usize> {
    let mut command = 0;
    loop {
        match program[command] {
            1 => intcode_add(program, command + 1, command + 2, command + 3),
            2 => intcode_multiply(program, command + 1, command + 2, command + 3),
            99 => break,
            _ => panic!(
                "Unexpected opcode {} at position {}",
                program[command], command
            ),
        }
        command += 4;
    }
    program
}

pub fn string_to_intcode(string: &str) -> Vec<usize> {
    use std::str::FromStr;
    string
        .split(",")
        .map(|x| usize::from_str(x.trim()).unwrap())
        .collect()
}
