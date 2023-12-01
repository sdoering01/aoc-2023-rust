struct LineNumIterator<'a> {
    line: &'a str,
    idx: usize,
}

impl<'a> LineNumIterator<'a> {
    fn new(line: &'a str) -> Self {
        Self { line, idx: 0 }
    }
}

impl<'a> Iterator for LineNumIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        fn get_num_from_start(s: &str) -> Option<u32> {
            let patterns = [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];

            if let Some(digit) = s.chars().next().and_then(|c| c.to_digit(10)) {
                return Some(digit);
            }

            for (pat, val) in patterns {
                if s.starts_with(pat) {
                    return Some(val);
                }
            }

            None
        }

        let mut item = None;
        while item.is_none() && self.idx < self.line.len() {
            item = get_num_from_start(&self.line[self.idx..]);
            self.idx += 1;
        }
        item
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut digits| {
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            10 * first + last
        })
        .sum()
}

fn part1_regex(input: &str) -> u32 {
    use regex::Regex;

    let re = Regex::new(r"\d").unwrap();

    input
        .lines()
        .map(|line| {
            let mut captures_iter = re.find_iter(line);
            let first = captures_iter.next().unwrap();
            let last = captures_iter.last().unwrap_or(first);

            let first_digit = first.as_str().chars().next().unwrap().to_digit(10).unwrap();
            let last_digit = last.as_str().chars().next().unwrap().to_digit(10).unwrap();

            10 * first_digit + last_digit
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(LineNumIterator::new)
        .map(|mut digits| {
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            10 * first + last
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input));
    dbg!(part1_regex(input));
    dbg!(part2(input));
    // Part 2 is not (easily) solvable with regexex, since the `regex` crate doesn't allow for
    // overlapping matches, and written out numbers can overlap (i.e. `oneight` = 18).
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../part1.txt");
        assert_eq!(part1(input), 142);
        assert_eq!(part1_regex(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../part2.txt");
        assert_eq!(part2(input), 281);
    }
}
