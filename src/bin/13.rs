advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let patterns = input.split("\n\n");
    let mirrors = patterns
        .map(|pattern| {
            let lines = pattern.lines().collect::<Vec<_>>();
            assert_ne!(lines.len() % 2, 0);
            assert_ne!(lines[0].len() % 2, 0);
            return if let Some(x) = get_vertical_mirror_position(&lines, 0) {
                x + 1
            } else {
                (get_horizontal_mirror_position(&lines, 0).unwrap() + 1) * 100
            };
        })
        .collect::<Vec<_>>();

    Some(mirrors.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let patterns = input.split("\n\n");
    let mirrors = patterns
        .map(|pattern| {
            let lines = pattern.lines().collect::<Vec<_>>();
            assert_ne!(lines.len() % 2, 0);
            assert_ne!(lines[0].len() % 2, 0);
            return if let Some(x) = get_vertical_mirror_position(&lines, 1) {
                x + 1
            } else {
                (get_horizontal_mirror_position(&lines, 1).unwrap() + 1) * 100
            };
        })
        .collect::<Vec<_>>();

    Some(mirrors.iter().sum())
}

fn get_horizontal_mirror_position(lines: &Vec<&str>, expected_wrong_count: usize) -> Option<usize> {
    let horizontal_mirrors = 0..lines.len() - 1;
    for row in horizontal_mirrors {
        let mut count_wrong = 0;
        lines.iter().enumerate().for_each(|(y, s)| {
            s.chars().enumerate().for_each(|(x, c)| {
                if y > row {
                } else {
                    let found = lines
                        .get(get_mirror(y, row))
                        .unwrap_or(&"")
                        .chars()
                        .nth(x)
                        .unwrap_or('&');
                    if found == '&' {
                    } else if c != found {
                        count_wrong += 1;
                    }
                }
            })
        });
        if count_wrong == expected_wrong_count {
            return Some(row);
        }
    }
    None
}

fn get_vertical_mirror_position(lines: &Vec<&str>, expected_wrong_count: usize) -> Option<usize> {
    let vertical_mirrors = 0..lines[0].len() - 1;
    for col in vertical_mirrors {
        let mut count_wrong = 0;
        lines.iter().enumerate().for_each(|(_, s)| {
            s.chars().enumerate().for_each(|(x, c)| {
                if x > col {
                } else {
                    let found = s.chars().nth(get_mirror(x, col)).unwrap_or('&');
                    if found == '&' {
                    } else if c != found {
                        count_wrong += 1;
                    }
                }
            })
        });
        if count_wrong == expected_wrong_count {
            return Some(col);
        }
    }
    None
}

fn get_mirror(a: usize, col_mirror: usize) -> usize {
    col_mirror.abs_diff(a) + col_mirror + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn get_col_mirror_test() {
        assert_eq!(get_mirror(3, 4), 6);
        assert_eq!(get_mirror(2, 4), 7);
        assert_eq!(get_mirror(1, 4), 8);
        assert_eq!(get_mirror(0, 4), 9);
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
