use regex::Regex;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    let re: Regex = Regex::new(r"\d+").unwrap();
    let cubed: Regex = Regex::new(r"\d+ \w+").unwrap();
    lines.for_each(|l|{
        let (cubes, game_id) = parse(l, &re, &cubed);
        if is_valid_(&cubes) {
            sum+= game_id;
        }
    });

    Some(sum)
}

fn parse(l:&str, re:&Regex, cubed:&Regex) -> (Vec<Vec<CubeSet>>, u32) {
    let mut games = l.split(";");
    let id_str = games.clone().next().unwrap();
    let game_id = re.find(id_str).unwrap().as_str().parse::<u32>().unwrap();
    let cubes: Vec<_> = games.map(|g|{
        return cubed.find_iter(g).map(|m|{
            let mut split_m = m.as_str().split(" ");
            let amount = split_m.next().unwrap();
            let color = split_m.next().unwrap();
            return CubeSet{ amount: amount.parse().unwrap(), color: color.to_string() };
        }).collect::<Vec<CubeSet>>()
    }).collect();
    return (cubes, game_id);
}
fn is_valid_(cubes: &Vec<Vec<CubeSet>>) -> bool {
    for c in cubes.iter() {
        for cube in c.iter() {
            if cube.color == "red" && cube.amount >12{
                return false;
            }
            if cube.color == "green" && cube.amount >13 {
                return false;
            }
            if cube.color == "blue" && cube.amount >14 {
                return false;
            }
        }
    }
    return true;
}
#[derive(Debug)]
struct CubeSet{
    amount: u32,
    color: String
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let re = Regex::new(r"\d+").unwrap();
    let cubed = Regex::new(r"\d+ \w+").unwrap();
    let mut sum = 0;
    lines.for_each(|l|{
        let (cubes,_) = parse(l, &re, &cubed);
        sum += min_power(&cubes);
    });

    Some(sum)
}

fn min_power(cubes: &Vec<Vec<CubeSet>>) -> u32 {
    let mut min_red = 1;
    let mut min_green = 1;
    let mut min_blue = 1;
    for c in cubes.iter() {
        for cube in c.iter() {
            if cube.color == "red"{
                min_red = min_red.max(cube.amount);
            }
            if cube.color == "green"{
                min_green= min_green.max(cube.amount);
            }
            if cube.color == "blue" {
                min_blue= min_blue.max(cube.amount);
            }
        }
    }
    return min_red * min_blue * min_green;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
