advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let patterns = input.split("\n\n");
    let mirrors = patterns
        .enumerate()
        .map(|(n, pattern)| {
            let lines = pattern.lines().collect::<Vec<_>>();
            assert_ne!(lines.len() % 2, 0);
            assert_ne!(lines[0].len() % 2, 0);
            if let Some(x) = get_mirror_pos(&lines) {
                return x + 1;
            } else {
                dbg!(n, &lines);
                return (get_mirror_pos_y(&lines).unwrap() + 1) * 100;
            }
        })
        .collect::<Vec<_>>();

    Some(mirrors.iter().sum())
}

fn get_mirror_pos_y_one_off(lines: &Vec<&str>) -> Option<usize> {
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
        if count_wrong == 1 {
            return Some(row);
        }
    }
    None
}

fn get_mirror_pos_y(lines: &Vec<&str>) -> Option<usize> {
    let horizontal_mirrors = 0..lines.len() - 1;
    for row in horizontal_mirrors {
        let mut winner = true;
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
                        winner = false
                    }
                }
            })
        });
        if winner {
            return Some(row);
        }
    }
    None
}

fn get_mirror_pos_one_off(lines: &Vec<&str>) -> Option<usize> {
    let vertical_mirrors = 0..lines[0].len() - 1;
    for col in vertical_mirrors {
        let mut count_wrong = 0;
        lines.iter().enumerate().for_each(|(y, s)| {
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
        if count_wrong == 1 {
            return Some(col);
        }
    }
    None
}

fn get_mirror_pos(lines: &Vec<&str>) -> Option<usize> {
    let vertical_mirrors = 0..lines[0].len() - 1;
    for col in vertical_mirrors {
        let mut winner = true;
        lines.iter().enumerate().for_each(|(y, s)| {
            s.chars().enumerate().for_each(|(x, c)| {
                if x > col {
                } else {
                    let found = s.chars().nth(get_mirror(x, col)).unwrap_or('&');
                    if found == '&' {
                    } else if c != found {
                        winner = false
                    }
                }
            })
        });
        if winner {
            return Some(col);
        }
    }
    None
}

fn get_mirror(a: usize, col_mirror: usize) -> usize {
    col_mirror.abs_diff(a) + col_mirror + 1
}

pub fn part_two(input: &str) -> Option<usize> {
    let patterns = input.split("\n\n");
    let mirrors = patterns
        .enumerate()
        .map(|(n, pattern)| {
            let lines = pattern.lines().collect::<Vec<_>>();
            assert_ne!(lines.len() % 2, 0);
            assert_ne!(lines[0].len() % 2, 0);
            if let Some(x) = get_mirror_pos_one_off(&lines) {
                return x + 1;
            } else {
                dbg!(n, &lines);
                return (get_mirror_pos_y_one_off(&lines).unwrap() + 1) * 100;
            }
        })
        .collect::<Vec<_>>();

    Some(mirrors.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn vertical_slice() {
        let x = get_mirror_pos(
            &"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
                .lines()
                .into_iter()
                .collect(),
        )
        .unwrap();
        assert_eq!(x, 4);
    }

    #[test]
    fn hori_slice() {
        let x = get_mirror_pos_y(
            &"...#.###...
###.##.##.#
#...#.##.#.
.####..####
##..###.#..
.#.#..#.#.#
#....####.#
#....####.#
.#.#..#.#.#
##..###.#..
.####..#.##
#...#.##.#.
###.##.##.#
...#.###...
...#.###..."
                .lines()
                .into_iter()
                .collect(),
        )
        .unwrap();
        assert_eq!(x, 13)
    }

    #[test]
    fn wtf() {
        let x = get_mirror_pos_y(
            &"..##..#.##..###
##.##.....##..#
##.##.....###.#
..##..#.##..###
..#.#.##.#...#.
...#.#..#.##.#.
.########....##
###...#.####.##
#...####...##.#
..........#....
##...###.......
########...#.#.
########...#.#."
                .lines()
                .into_iter()
                .collect(),
        )
        .unwrap();
        assert_eq!(x, 11)
    }

    #[test]
    fn wtf_2() {
        let x = get_mirror_pos(
            &"....#....
##..#....
...#...#.
....#..##
##.#.###.
###...###
..#...#..
######.##
##.###.##
..#...#..
###...###
##.#.###.
....#..##"
                .lines()
                .into_iter()
                .collect(),
        )
        .unwrap();
        assert_eq!(x, 0)
    }
    #[test]
    fn part_one_testb() {
        let x = get_mirror_pos_y(
            &"###..#....##.....
##...##.#####.#.#
#..###.#....##.##
.#.###.#.#.##...#
.#.###.#.#.##...#
#..#####....##.##
##...##.#####.#.#
###..#....##.....
.##....######.##.
..#######..##.###
#.##......#.##.##
.#..##.#....#.##.
#.##.###.##..#.#.
..#..#..##.###..#
..#..#..##.###..#
#.##.###.##..#.#.
.#..##.#....#.##."
                .lines()
                .into_iter()
                .collect(),
        )
        .unwrap();
        assert_eq!(x, 13);
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
