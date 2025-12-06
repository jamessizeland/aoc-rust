use aoc::prelude::*;
#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    // count times the dial stops on 0.
    let mut output = 0;
    let mut dial_pos = 50;
    let lines = input.lines().collect_vec();
    for line in lines {
        let (dir, dist) = line.split_at(1);
        let clicks: isize = match dir {
            "L" => -dist.parse()?,
            "R" => dist.parse()?,
            _ => panic!("expected L or R"),
        };
        println!("{dir} {dist}");
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
    let lines = input.lines().collect_vec();
    let output = lines.len();
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
        assert_eq!(result, 0);
        todo!("set up test");
    }
}
