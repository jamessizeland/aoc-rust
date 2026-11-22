use aoc::prelude::*;
#[allow(unused)]
fn part_a(input: &Input) -> Result<usize> {
    let mut output = 0;
    let map = input.to_grid();
    for x in 0..map.width() {
        for y in 0..map.height() {
            let mut num_rolls = 0;
            let point = Point::new(x, y);
            if map.get(point) != Some('@') {
                continue; // only count if this is a roll space
            };
            let neighbours = point.neighbors_extended();
            for neighbour in neighbours {
                if map.get(neighbour) == Some('@') {
                    num_rolls += 1;
                }
                if num_rolls > 4 {
                    break; // too many rolls adjacent
                }
            }
            if num_rolls <= 4 {
                output += 1;
            }
        }
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
    println!("part a: {}", part_a(&input).unwrap());
    #[cfg(feature = "b")]
    println!("part b: {}", part_b(&input).unwrap());
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    #[test]
    fn test_part_a() {
        let input = Input::from_literal(INPUT);
        let result = part_a(&input).unwrap();
        assert_eq!(result, 13);
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
