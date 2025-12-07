use aoc::prelude::*;

fn colour_indexes(input: &str, indexes: &[usize]) {
    let chars = input.chars().collect_vec();
    for (idx, ch) in chars.iter().enumerate() {
        let s = ch.to_string();
        if indexes.contains(&idx) {
            print!("{}", s.red());
        } else {
            print!("{}", s.green());
        }
    }
}
fn combine_nums(nums: &(u32, u32)) -> Result<usize> {
    Ok(format!("{}{}", nums.0, nums.1).parse()?)
}

#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    let lines = input.lines().collect_vec();
    for line in lines {
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
        let mut largest_joltage = 0;
        let mut refs = (0, 0);
        'outer: for pos in 0..(digits.len() - 1) {
            let first = digits[pos];
            let mut largest = (first, 0); // reset second value
            let mut new_refs = (pos, 0);
            for pos in (pos + 1)..digits.len() {
                let second = digits[pos];
                if second > largest.1 {
                    largest.1 = second;
                    new_refs.1 = pos;
                }
            }
            let new_joltage = combine_nums(&largest)?;
            if new_joltage >= largest_joltage {
                largest_joltage = new_joltage;
                refs = new_refs;
            }
        }
        colour_indexes(line, &[refs.0, refs.1]);
        println!(" -> {largest_joltage}");
        output += largest_joltage;
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
        let input = Input::from_literal("8586");
        let result = part_a(&input).unwrap();
        assert_eq!(result, 88);
    }

    #[test]
    fn test_part_b() {
        let input = Input::from_literal(INPUT);
        let result = part_b(&input).unwrap();
        assert_eq!(result, 0);
        todo!("set up test");
    }
}
