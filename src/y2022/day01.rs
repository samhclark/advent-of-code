#[allow(dead_code)]
pub fn part01() {
    let puzzle_input = include_str!("../../inputs/2022/day01.in");
    println!("Answer is: {}", cals_of_top_n_elves(puzzle_input, 1));
}

#[allow(dead_code)]
pub fn part02() {
    let puzzle_input = include_str!("../../inputs/2022/day01.in");
    println!("Answer is: {}", cals_of_top_n_elves(puzzle_input, 3));
}

fn cals_of_top_n_elves(input: &str, n: usize) -> u64 {
    let mut elf_cals: Vec<u64> = vec![];
    let mut current_elf_cals: u64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            elf_cals.push(current_elf_cals);
            current_elf_cals = 0;
        } else {
            current_elf_cals += line.parse::<u64>().unwrap();
        }
    }
    elf_cals.push(current_elf_cals);

    elf_cals.sort_unstable();
    let top_n_elves = elf_cals.drain((elf_cals.len() - n)..);
    top_n_elves.sum()
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
