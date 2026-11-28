use std::ops::Range;

use aoc::prelude::*;

fn parse_ranges(input: &str) -> Result<Vec<Range<usize>>> {
    input
        .split(',')
        .map(|range| {
            let Some((low, high)) = range.split('-').collect_tuple() else {
                return Err(anyhow!("incompatible range found"));
            };
            Ok(Range {
                start: low.parse()?,
                end: high.parse::<usize>()? + 1,
            })
        })
        .collect()
}

#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    let lines = input.lines().collect_vec();
    assert!(lines.len() == 1);
    let ranges = parse_ranges(lines[0])?;
    for range in ranges {
        for id in range {
            let digits = id.to_string().chars().collect_vec();
            let (first, second) = digits.split_at(digits.len() / 2);
            if first == second {
                output += id;
            }
        }
    }
    Ok(output)
}
#[allow(unused)]
fn part_b(input: &Input) -> Result<usize> {
    let mut output = 0;
    let lines = input.lines().collect_vec();
    assert!(lines.len() == 1);
    let ranges = parse_ranges(lines[0])?;
    for range in ranges {
        for id in range {
            let digits = id.to_string().chars().collect_vec();
            // split the digits into equal parts of increasing length
            // (up to halves) and check for equality for an invalid ID.
            for pattern_len in 1..=(digits.len() / 2) {
                let is_divisible = (digits.len() % pattern_len) == 0;
                if is_divisible && digits.chunks_exact(pattern_len).all_equal() {
                    output += id;
                    break;
                }
            }
        }
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
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part_a() {
        let input = Input::from_literal(INPUT);
        let result = part_a(&input).unwrap();
        assert_eq!(result, 1227775554);
    }
    #[test]
    fn test_part_b() {
        let input = Input::from_literal(INPUT);
        let result = part_b(&input).unwrap();
        assert_eq!(result, 4174379265);
    }
}
