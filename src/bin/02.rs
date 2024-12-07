use core::panic;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // let mut records: Vec<Vec<i32>> = vec![];
    let reports = input.lines().map(|l| {
        l.split(" ")
            .map(|u| u.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let count = reports.map(check_safe).filter(|r| r.eq(&true)).count();
    Some(count.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.lines().map(|l| {
        l.split(" ")
            .map(|u| u.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let count = reports
        .map(dampened_check_safe)
        .filter(|r| r.eq(&true))
        .count();
    Some(count as u32)
}

fn dampened_check_safe(report: Vec<i32>) -> bool {
    match check_safe(report.clone()) {
        true => true,
        false => {
            for i in 0..report.len() {
                let mut clone = report.clone();
                clone.remove(i);

                if check_safe(clone) {
                    return true;
                }
            }
            false
        }
    }
}

fn check_safe(report: Vec<i32>) -> bool {
    let mut signo = 0;
    for i in 1..report.len() {
        let dx = report[i] - report[i - 1];
        if signo == 0 {
            signo = dx.signum();
        }
        match signo * dx.signum() {
            -1 | 0 => {
                return false;
            }
            1 => {}
            _ => {}
        }
        if dx.abs() > 3 {
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
