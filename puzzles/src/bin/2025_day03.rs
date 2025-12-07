use aoc::prelude::*;
#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    let lines = input.lines().collect_vec();
    for line in lines {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
        let mut largest = (0, 0);
        'outer: for pos in 0..(digits.len() - 1) {
            let first = digits[pos];
            if first < largest.0 {
                continue;
            }
            largest = (first, 0); // reset second value
            // println!("first -> {first}");
            for pos in (pos + 1)..digits.len() {
                let second = digits[pos];
                // println!("second -> {second}");
                if second > largest.1 {
                    largest.1 = second;
                    if first == 9 && second == 9 {
                        break 'outer; // found max possible Joltage
                    }
                }
            }
        }
        let joltage: usize = format!("{}{}", largest.0, largest.1).parse()?;
        println!("{line} -> {joltage}");
        output += joltage;
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
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    #[test]
    fn test_part_a() {
        let input = Input::from_literal(INPUT);
        let result = part_a(&input).unwrap();
        assert_eq!(result, 357);
    }
    #[test]
    fn test_part_a2() {
        let input = Input::from_literal("9798");
        let result = part_a(&input).unwrap();
        assert_eq!(result, 99);
    }

    #[test]
    fn test_part_b() {
        let input = Input::from_literal(INPUT);
        let result = part_b(&input).unwrap();
        assert_eq!(result, 0);
        todo!("set up test");
    }
}
