use std::cmp::max_by;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let chars = parse_input(input);
    let mut instances = 0;
    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            if chars[i][j] == 'X' {
                // print!("({},{})", i + 1, j + 1);
                let minx: i32 = if i < 3 { 0 } else { -1 };
                let miny: i32 = if j < 3 { 0 } else { -1 };
                let maxx = if i >= chars.len() - 3 { 0 } else { 1 };
                let maxy = if j >= chars[0].len() - 3 { 0 } else { 1 };
                // println!(" {}, {}, {}, {}", minx, maxx, miny, maxy);

                for dx in minx..=maxx {
                    'parsing: for dy in miny..=maxy {
                        let mut buffer = vec![];
                        if dx == 0 && dy == 0 {
                            continue 'parsing;
                        }
                        for s in 0..4 {
                            buffer.push(
                                chars[usize::try_from(i as i32 + dx * s).unwrap()]
                                    [usize::try_from(j as i32 + dy * s).unwrap()],
                            );
                        }
                        if check_xmas(&buffer) {
                            instances += 1;
                        }
                    }
                }
            }
        }
    }
    Some(instances)
}

fn check_xmas(buffer: &[char]) -> bool {
    let pattern = ['X', 'M', 'A', 'S'];
    let pattern_rev = ['S', 'A', 'M', 'X'];

    // for b in buffer {
    //     print!("{b}");
    // }
    //
    // println!(" {}", *buffer == pattern);

    if *buffer == pattern {
        return true;
    }
    if *buffer == pattern_rev {
        return true;
    }
    false
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => 'X',
                    'M' => 'M',
                    'A' => 'A',
                    'S' => 'S',
                    _ => '.',
                })
                .collect()
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars = parse_input(input);

    let mut instances = 0;
    for i in 1..chars.len() - 1 {
        for j in 1..chars[0].len() - 1 {
            if chars[i][j] == 'A'
                && check_x_mas(&[
                    &[chars[i - 1][j - 1], chars[i - 1][j + 1]],
                    &[chars[i + 1][j - 1], chars[i + 1][j + 1]],
                ])
            {
                instances += 1;
            }
        }
    }
    Some(instances)
}

fn check_x_mas(buffer: &[&[char]]) -> bool {
    match [buffer[0][0], buffer[1][1]] {
        ['M', 'S'] | ['S', 'M'] => {}
        _ => {
            return false;
        }
    }
    match [buffer[1][0], buffer[0][1]] {
        ['M', 'S'] | ['S', 'M'] => {}
        _ => {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
