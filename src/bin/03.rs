advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_memory).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut flag = true;
    for l in input.lines() {
        let tup = parse_memory_conditionals(l, flag);
        flag = tup.1;
        sum += tup.0;
    }
    Some(sum)
}

fn parse_memory(line: &str) -> u32 {
    let mut positions: Vec<_> = line.match_indices("mul(").collect();
    let chars: Vec<_> = line.chars().collect();

    if positions.is_empty() {
        return 0;
    }
    let ultimo = positions.last().unwrap().0;
    if ultimo + 7 >= chars.len() {
        positions.remove(positions.len() - 1);
    }

    let mut suma = 0;
    for p in positions {
        let mut i = p.0 + 4;
        while chars[i].is_numeric() {
            i += 1;
        }

        let n1: u32 = line[p.0 + 4..i].parse().unwrap();

        if chars[i] != ',' {
            continue;
        }
        i += 1;
        let s2 = i;

        while chars[i].is_numeric() {
            i += 1;
        }

        let n2: u32 = line[s2..i].parse().unwrap();
        if chars[i] != ')' {
            continue;
        }

        suma += n1 * n2;
    }

    suma
}

fn parse_memory_conditionals(line: &str, flag: bool) -> (u32, bool) {
    let mut positions: Vec<_> = line.match_indices("mul(").collect();
    let mut conditionals: Vec<_> = line.match_indices("do()").map(|(i, _)| (i, true)).collect();
    let donts: Vec<_> = line
        .match_indices("don't()")
        .map(|(i, _)| (i, false))
        .collect();

    conditionals.extend(donts);
    conditionals.sort();

    let chars: Vec<_> = line.chars().collect();

    if positions.is_empty() {
        return (0, flag);
    }

    let ultimo = positions.last().unwrap().0;
    if ultimo + 7 >= chars.len() {
        positions.remove(positions.len() - 1);
    }

    let mut suma = 0;
    let mut enable = flag;
    let mut cond = conditionals.iter();
    let mut next_conditional = Some(cond.next().unwrap());
    for p in positions {
        if let Some(c) = next_conditional {
            if p.0 > c.0 {
                enable = c.1;
                next_conditional = cond.next();
            }
        };

        if !enable {
            continue;
        }
        let mut i = p.0 + 4;
        while chars[i].is_numeric() {
            i += 1;
        }

        let n1: u32 = line[p.0 + 4..i].parse().unwrap();

        if chars[i] != ',' {
            continue;
        }
        i += 1;
        let s2 = i;

        while chars[i].is_numeric() {
            i += 1;
        }

        let n2: u32 = line[s2..i].parse().unwrap();
        if chars[i] != ')' {
            continue;
        }

        suma += n1 * n2;
    }

    (suma, enable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
