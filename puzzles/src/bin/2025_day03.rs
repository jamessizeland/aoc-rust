use aoc::prelude::*;

#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    for line in input.lines() {
        let digits = line.digits();
        let mut largest_joltage = 0;
        let mut refs: [usize; 2] = [0; 2];
        for pos in 0..(digits.len() - 1) {
            let first = digits[pos];
            let mut largest = (first, 0); // reset second value
            let mut new_refs = [pos, 0];
            for pos in (pos + 1)..digits.len() {
                let second = digits[pos];
                if second > largest.1 {
                    largest.1 = second;
                    new_refs[1] = pos;
                }
            }
            let new_joltage = largest.combine()?;
            if new_joltage >= largest_joltage {
                largest_joltage = new_joltage;
                refs = new_refs;
            }
        }
        println!("{} -> {largest_joltage}", line.colour_by_index(&refs));
        output += largest_joltage;
    }

    Ok(output)
}
#[allow(unused)]
fn part_b(input: &Input) -> Result<usize> {
    let mut output = 0;
    for line in input.lines() {
        let digits = line.digits();
        let mut largest_joltage = 0;
        let mut refs: [usize; 12] = [0; 12]; // indexes of largest joltage combo
        println!("{} -> {largest_joltage}", line.colour_by_index(&refs));
        output += largest_joltage;
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
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_part_b2() {
        let input = Input::from_literal("818181711872111");
        let result = part_b(&input).unwrap();
        assert_eq!(result, 888911112111);
    }
}
