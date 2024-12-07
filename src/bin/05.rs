use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (reglas, lineas) = parse_state(input);

    Some(
        lineas
            .into_iter()
            .filter(|l| check_line(l.to_vec(), &reglas))
            .map(|c| c[c.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (reglas, lineas) = parse_state(input);
    let incorrectas: u32 = lineas
        .into_iter()
        .filter(|l| !check_line(l.to_vec(), &reglas))
        .map(|mut l| {
            l.sort_by(|a, b| compare(*a, *b, &reglas));
            return l;
        })
        .map(|c| c[c.len() / 2])
        .sum();
    Some(incorrectas)
}

fn fix_line(line: &mut [u32], reglas: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    for i in 0..line.len() {
        let infracciones = match reglas.get(&line[i]) {
            Some(v) => v,
            None => {
                continue;
            }
        };
        for j in 0..i {
            for inf in infracciones {
                if line[j] == *inf {
                    line.into_iter().for_each(|t| {
                        print!("{t},");
                    });
                    println!();
                    for t in (j..i).rev() {
                        line.swap(t, t + 1)
                    }
                }
            }
        }
    }
    line.to_vec()
}

fn compare(a: u32, b: u32, reglas: &HashMap<u32, Vec<u32>>) -> Ordering {
    match reglas.get(&a) {
        Some(v) => {
            if v.contains(&b) {
                return Ordering::Less;
            }
        }
        None => {
            return Ordering::Greater;
        }
    }

    match reglas.get(&b) {
        Some(v) => {
            if v.contains(&a) {
                return Ordering::Greater;
            }
        }
        None => {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn check_line(line: Vec<u32>, reglas: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..line.len() {
        let infracciones = match reglas.get(&line[i]) {
            Some(v) => v,
            None => {
                continue;
            }
        };
        for j in 0..i {
            for inf in infracciones {
                if line[j] == *inf {
                    return false;
                }
            }
        }
    }
    true
}

fn parse_state(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut reglas: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lineas: Vec<Vec<u32>> = vec![];

    let mut band = false;
    for l in input.lines() {
        if l.is_empty() {
            band = true;
            continue;
        }
        if !band {
            let v: Vec<_> = l.split("|").collect();
            let k = &v[0].parse::<u32>().unwrap();
            match reglas.get_mut(k) {
                Some(sucesores) => {
                    sucesores.push(v[1].parse().unwrap());
                }
                None => {
                    reglas.insert(*k, vec![v[1].parse().unwrap()]);
                }
            }

            continue;
        }

        lineas.push(l.split(",").map(|n| n.parse::<u32>().unwrap()).collect());
    }

    (reglas, lineas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
