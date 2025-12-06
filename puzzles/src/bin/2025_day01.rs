use aoc::prelude::*;

fn parse_clicks(line: &str) -> Result<isize> {
    let (dir, dist) = line.split_at(1);
    let count: isize = match dir {
        "L" => -dist.parse()?,
        "R" => dist.parse()?,
        _ => panic!("expected L or R"),
    };
    Ok(count)
}
#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    // count times the dial stops on 0.
    let mut output = 0;
    let mut dial_pos = 50;
    let lines = input.lines().collect_vec();
    for line in lines {
        let clicks = parse_clicks(line)?;
        dial_pos += clicks;
        while dial_pos.is_negative() || dial_pos > 99 {
            if dial_pos.is_negative() {
                dial_pos = 100 + dial_pos;
            } else if dial_pos > 99 {
                dial_pos = dial_pos - 100;
            }
        }
        if dial_pos == 0 {
            output += 1;
        }
        println!("{dial_pos}");
    }
    Ok(output)
}
#[allow(unused)]
fn part_b(input: &Input) -> Result<usize> {
    // count times the dial stops on *or passes through* 0.
    let mut output = 0;
    let mut dial_pos = 50;
    for line in input.lines() {
        let clicks = parse_clicks(line)?;

        // Each full rotation passes through 0 once.
        let full_rotations = clicks.abs() / 100;
        output += full_rotations as usize;

        let start_pos = dial_pos;
        let remaining_clicks = clicks % 100;
        let end_pos = start_pos + remaining_clicks;

        // Check if the remaining movement crosses 0.
        // This happens if we move from one side of 0 to the other.
        // The start position is not counted as a "pass", but the end is.
        if remaining_clicks > 0 && start_pos < 100 && end_pos >= 100 {
            // Pass 99->0
            output += 1;
        } else if remaining_clicks < 0 && start_pos > 0 && end_pos <= 0 {
            // Pass 0->99
            output += 1;
        }
        dial_pos = end_pos.rem_euclid(100);
    }
    Ok(output)
}
fn main() {
    let input = Input::from_puzzle();
    #[cfg(feature = "a")]
    println!("part a: {}", part_a(&input).unwrap());
    #[cfg(feature = "b")]
    println!("part b: {}", part_b(&input).unwrap());
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn test_part_a() {
        let input = Input::from_literal(INPUT);
        let result = part_a(&input).unwrap();
        assert_eq!(result, 3);
    }
    #[test]
    fn test_part_b() {
        let input = Input::from_literal(INPUT);
        let result = part_b(&input).unwrap();
        assert_eq!(result, 6);
    }
    #[test]
    fn test_part_b2() {
        let input = Input::from_literal("R1000");
        let result = part_b(&input).unwrap();
        assert_eq!(result, 10);
    }
}
