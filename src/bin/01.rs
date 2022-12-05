use std::ops::Deref;

fn populate_elf_calories(input: &str) -> Vec<u32> {
    let mut sums: Vec<u32> = vec![];
    let mut s: u32 = 0;

    for l in input.lines() {
        if l.is_empty() {
            sums.push(s);
            s = 0;
            continue
        }
        s = s + l.parse::<u32>().unwrap();
    }
    sums.push(s);

    sums
}
pub fn part_one(input: &str) -> Option<u32> {
    let elf_calories = populate_elf_calories(input);
    elf_calories.iter().max().cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories = populate_elf_calories(input);
    elf_calories.sort_by(|x,y| y.cmp(x));
    Some(elf_calories.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
