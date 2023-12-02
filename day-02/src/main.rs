use std::collections::BTreeMap;

#[derive(Default, Debug)]
struct CubeSet {
    r: u8,
    g: u8,
    b: u8,
}

impl From<&str> for CubeSet {
    fn from(set_str: &str) -> CubeSet {
        let mut cube_set = CubeSet::default();
        for set_part_str in set_str.split(", ") {
            let mut set_part_iter = set_part_str.splitn(2, ' ');
            let amount: u8 = set_part_iter.next().unwrap().parse().unwrap();
            let color = set_part_iter.next().unwrap();
            match color {
                "red" => cube_set.r = amount,
                "green" => cube_set.g = amount,
                "blue" => cube_set.b = amount,
                _ => unreachable!(),
            };
        }
        cube_set
    }
}

fn parse_input(input: &str) -> BTreeMap<u8, Vec<CubeSet>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, ": ");
            let game: u8 = parts
                .next()
                .unwrap()
                .trim_start_matches("Game ")
                .parse()
                .unwrap();

            let cube_sets: Vec<_> = parts
                .next()
                .unwrap()
                .split("; ")
                .map(CubeSet::from)
                .collect();

            (game, cube_sets)
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let games = parse_input(input);
    let bag_set = CubeSet {
        r: 12,
        g: 13,
        b: 14,
    };

    games
        .into_iter()
        .filter(|(_, cube_sets)| {
            cube_sets.iter().all(|cube_set| {
                cube_set.r <= bag_set.r && cube_set.g <= bag_set.g && cube_set.b <= bag_set.b
            })
        })
        .map(|(num, _)| u64::from(num))
        .sum()
}

fn part2(input: &str) -> u64 {
    let games = parse_input(input);

    games
        .into_values()
        .map(|cube_sets| {
            let mut max_set = CubeSet::default();
            for cube_set in cube_sets {
                max_set.r = u8::max(max_set.r, cube_set.r);
                max_set.g = u8::max(max_set.g, cube_set.g);
                max_set.b = u8::max(max_set.b, cube_set.b);
            }
            max_set
        })
        .map(|cube_set| u64::from(cube_set.r) * u64::from(cube_set.g) * u64::from(cube_set.b))
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
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../part2.txt");
        assert_eq!(part2(input), 2286);
    }
}
