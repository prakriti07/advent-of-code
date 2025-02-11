use super::int_code::{Program, Word};
use crate::input::Input;

fn affected_by_beam(program: &Program, x: i32, y: i32) -> Result<bool, String> {
    let mut program_copy = program.clone();
    program_copy.input(Word::from(x));
    program_copy.input(Word::from(y));
    let output = program_copy.run_for_output()?;
    if output.is_empty() {
        return Err("No output produced".to_string());
    } else if output.len() != 1 || !matches!(output[0], 0 | 1) {
        return Err("Invalid output from program (expected only 0 or 1)".to_string());
    }
    Ok(output[0] == 1)
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    const MAX_COORDINATE: i32 = 10_000;
    let program = Program::parse(input.text)?;

    let is_part_one = input.is_part_one();

    if is_part_one {
        let mut affected_count = 0;
        for (x, y) in (0..50).flat_map(|x| (0..50).map(move |y| (x, y))) {
            if affected_by_beam(&program, x, y)? {
                affected_count += 1;
            }
        }

        Ok(affected_count)
    } else {
        // Find the initial start of the beam (skipping (0,0),
        // as it is not connected to rest of the beam.
        let mut bottom_edge = 100;
        let mut left_edge = 0;

        while bottom_edge < MAX_COORDINATE {
            // Proceed down line by line:
            bottom_edge += 1;

            // Walk right until we are affected by the beam:
            while !affected_by_beam(&program, left_edge, bottom_edge)? {
                left_edge += 1;
                if left_edge >= MAX_COORDINATE {
                    return Err(format!("Aborting after reaching x={}", MAX_COORDINATE));
                }
            }

            // Check if square fits:
            let right_edge = left_edge + 99;
            let top_edge = bottom_edge - 99;
            if affected_by_beam(&program, right_edge, top_edge)? {
                return Ok((left_edge * 10000 + top_edge) as u32);
            }
        }

        return Err(format!("Aborting after reaching y={}", MAX_COORDINATE));
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let input = include_str!("day19_input.txt");
    test_part_one!(input => 112);
    test_part_two!(input => 18_261_982);

    let other_input = "109,424,203,1,21102,11,1,0,1106,0,282,21102,18,1,0,1106,0,259,1201,1,0,221,203,1,21102,31,1,0,1106,0,282,21101,38,0,0,1106,0,259,21002,23,1,2,22102,1,1,3,21101,1,0,1,21102,57,1,0,1105,1,303,2102,1,1,222,20101,0,221,3,20101,0,221,2,21102,259,1,1,21101,80,0,0,1106,0,225,21101,0,44,2,21102,91,1,0,1105,1,303,1201,1,0,223,20101,0,222,4,21101,0,259,3,21102,225,1,2,21101,225,0,1,21102,118,1,0,1105,1,225,21002,222,1,3,21101,100,0,2,21101,133,0,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21101,148,0,0,1106,0,259,2102,1,1,223,20102,1,221,4,21002,222,1,3,21102,1,12,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,1,195,0,106,0,108,20207,1,223,2,21002,23,1,1,21102,-1,1,3,21101,0,214,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,2102,1,-4,249,21201,-3,0,1,22101,0,-2,2,22101,0,-1,3,21101,0,250,0,1105,1,225,22102,1,1,-4,109,-5,2106,0,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2106,0,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,21202,-2,1,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,22102,1,-2,3,21101,0,343,0,1105,1,303,1105,1,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,21201,-4,0,1,21101,0,384,0,1106,0,303,1106,0,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,22102,1,1,-4,109,-5,2106,0,0";
    test_part_one!(other_input => 147);
    test_part_two!(other_input => 13_280_865);
}
