use aoc::prelude::*;
#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    for line in input.lines() {
        println!("{}", line);
    }
    Ok(output)
}
#[allow(unused)]
fn part_b(input: &Input) -> Result<usize> {
    let mut output = 0;
    for line in input.lines() {
        println!("{}", line);
    }
    Ok(output)
}
fn main() {
    let input = Input::from_puzzle();
    #[cfg(feature = "a")]
    println!("part a: {}", part_a(& input).unwrap());
    #[cfg(feature = "b")]
    println!("part b: {}", part_b(& input).unwrap());
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    #[test]
    fn test_part_a() {
        let input = Input::from_literal(INPUT);
        let result = part_a(&input).unwrap();
        assert_eq!(result, 0);
        todo!("set up test");
    }
    #[test]
    fn test_part_b() {
        let input = Input::from_literal(INPUT);
        let result = part_b(&input).unwrap();
        assert_eq!(result, 0);
        todo!("set up test");
    }
}
