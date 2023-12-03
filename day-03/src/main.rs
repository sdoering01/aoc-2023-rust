use std::collections::BTreeMap;

struct NumHit {
    num: u64,
    row: usize,
    col_start: usize,
    col_end: usize,
}

fn get_num_hits(engine: &[Vec<char>]) -> Vec<NumHit> {
    let mut num_hits: Vec<NumHit> = Vec::new();
    for (row, line) in engine.iter().enumerate() {
        let mut start_idx = 0;
        let mut end_idx;
        while start_idx < line.len() {
            if line[start_idx].is_ascii_digit() {
                end_idx = start_idx;
                while end_idx < line.len() && line[end_idx].is_ascii_digit() {
                    end_idx += 1;
                }

                let num: u64 = line[start_idx..end_idx]
                    .iter()
                    .fold(0, |acc, c| 10 * acc + c.to_digit(10).unwrap())
                    .into();

                num_hits.push(NumHit {
                    num,
                    row,
                    col_start: start_idx,
                    col_end: end_idx - 1,
                });

                start_idx = end_idx;
            } else {
                start_idx += 1;
            }
        }
    }
    num_hits
}

fn part1(input: &str) -> u64 {
    let engine: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    get_num_hits(&engine)
        .into_iter()
        .filter(|num_hit| {
            for row in (num_hit.row.saturating_sub(1))..=(num_hit.row + 1) {
                for col in (num_hit.col_start.saturating_sub(1))..=(num_hit.col_end + 1) {
                    if row < engine.len()
                        && col < engine[row].len()
                        && !engine[row][col].is_ascii_digit()
                        && engine[row][col] != '.'
                    {
                        return true;
                    }
                }
            }
            false
        })
        .map(|num_hit| num_hit.num)
        .sum()
}

fn part2(input: &str) -> u64 {
    let engine: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut gear_hits: BTreeMap<(usize, usize), Vec<u64>> = BTreeMap::new();

    'num_hit_loop: for num_hit in get_num_hits(&engine) {
        for row in (num_hit.row.saturating_sub(1))..=(num_hit.row + 1) {
            for col in (num_hit.col_start.saturating_sub(1))..=(num_hit.col_end + 1) {
                if row < engine.len() && col < engine[row].len() && engine[row][col] == '*' {
                    gear_hits.entry((row, col)).or_default().push(num_hit.num);
                    continue 'num_hit_loop;
                }
            }
        }
    }

    gear_hits
        .into_values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.into_iter().product::<u64>())
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../part1.txt");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../part2.txt");
        assert_eq!(part2(input), 467835);
    }
}
