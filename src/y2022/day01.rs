use itertools::Itertools;

static Y2022_DAY01_INPUT: &str = include_str!("../../inputs/2022/day01.in");

#[allow(dead_code)]
pub fn part01() {
    println!("Answer is: {}", cals_of_top_n_elves(Y2022_DAY01_INPUT, 1));
}

#[allow(dead_code)]
pub fn part02() {
    println!("Answer is: {}", cals_of_top_n_elves(Y2022_DAY01_INPUT, 3));
}

fn cals_of_top_n_elves(input: &str, n: usize) -> u64 {
    let mut elf_cals: Vec<u64> = vec![];
    let mut current_elf_cals: u64;
    for elf in input.split("\n\n") {
        current_elf_cals = 0;
        for food in elf.lines() {
            current_elf_cals += food.parse::<u64>().unwrap();
        }
        elf_cals.push(current_elf_cals);
    }
    elf_cals.iter().sorted().rev().take(n).sum::<u64>()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000\n";
        assert_eq!(cals_of_top_n_elves(input, 1), 24000 as u64);
    }

    #[test]
    fn test_case_part_2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000\n";
        assert_eq!(cals_of_top_n_elves(input, 3), 45000 as u64);
    }
}
