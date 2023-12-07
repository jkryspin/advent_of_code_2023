use crate::HandType::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(7);

fn sort(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        let sort = a
            .strongest_type
            .relative_value()
            .cmp(&b.strongest_type.relative_value());
        if sort == Ordering::Equal {
            for i in 0..a.cards.len() {
                let a = a.cards_strength[i];
                let b = b.cards_strength[i];
                if a < b {
                    return Ordering::Less;
                }
                if a > b {
                    return Ordering::Greater;
                }
            }
            panic!("none should match");
        }
        return sort;
    });
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| Hand::new(l, true))
        .collect::<Vec<_>>();
    sort(&mut hands);
    let mut sum = 0;
    let mut seen: HashSet<u32> = Default::default();
    hands.iter().enumerate().for_each(|(size, h)| {
        seen.insert(h.strongest_type.relative_value());
        let delta = (size as u32 + 1) * h.bid;
        sum += delta;
    });

    Some(sum)
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    cards_strength: Vec<u16>,
    bid: u32,
    strongest_type: HandType,
}

impl Hand {
    fn new(s: &str, part: bool) -> Self {
        let (l, r) = s.split_once(" ").unwrap();
        let bid = r.parse::<u32>().unwrap();
        let strongest_type = Hand::strongest_type(l, part);

        let map = if part {
            vec![
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ]
        } else {
            vec![
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ]
        };
        Self {
            cards: l.chars().collect(),
            cards_strength: l
                .chars()
                .map(|c| map.iter().position(|&x| x == c).unwrap() as u16)
                .collect(),
            bid,
            strongest_type,
        }
    }

    fn strongest_type(s: &str, part: bool) -> HandType {
        let mut map = HashMap::<char, u32>::new();
        for c in s.chars() {
            map.insert(c, map.get(&c).unwrap_or(&0).clone() + 1);
        }
        let mut joker_count = map.get(&'J').unwrap_or(&0).clone();
        if part {
            joker_count = 0;
        }
        if joker_count == 5 {
            return FiveOfKind;
        }

        let mut v = map.iter().collect::<Vec<_>>();
        v.sort_by(|a, b| a.0.cmp(b.0));
        if !part {
            map.remove(&'J');
        }
        for (c, val) in map.iter() {
            if val + joker_count == 5 {
                return FiveOfKind;
            }
        }

        for (c, val) in map.iter() {
            if val + joker_count == 4 {
                return FourOfKind;
            }
        }

        for (c, val) in map.iter() {
            for other in map
                .iter()
                .filter(|(other_c, other_val)| other_c != &c)
                .into_iter()
            {
                if val + joker_count == 3 && other.1 == &2 {
                    return FullHouse;
                }
            }
        }

        for (_, val) in map.iter() {
            if val + joker_count == 3 {
                return ThreeOfKind;
            }
        }

        for (c, val) in map.iter() {
            for other in map
                .iter()
                .filter(|(other_c, other_val)| other_c != &c)
                .into_iter()
            {
                if val + joker_count == 2 && other.1 == &2 {
                    return TwoPair;
                }
            }
        }

        for (c, val) in map.iter() {
            for other in map
                .iter()
                .filter(|(other_c, other_val)| other_c != &c)
                .into_iter()
            {
                if val + joker_count == 2 {
                    return OnePair;
                }
            }
        }
        return HighCard;
        panic!("No pair matched card!!")
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn relative_value(&self) -> u32 {
        return match self {
            HandType::FiveOfKind => 100,
            HandType::FourOfKind => 90,
            HandType::FullHouse => 80,
            HandType::ThreeOfKind => 70,
            HandType::TwoPair => 60,
            HandType::OnePair => 50,
            HandType::HighCard => 40,
        };
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| Hand::new(l, false))
        .collect::<Vec<_>>();
    sort(&mut hands);
    let mut sum = 0;
    let mut seen: HashSet<u32> = Default::default();
    hands.iter().enumerate().for_each(|(size, h)| {
        seen.insert(h.strongest_type.relative_value());
        let delta = (size as u32 + 1) * h.bid;
        sum += delta;
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn type_checker() {
        let t = Hand::strongest_type("33332", true);
        assert_eq!(t, FourOfKind);
        let t = Hand::strongest_type("33333", true);
        assert_eq!(t, FiveOfKind);

        let t = Hand::strongest_type("23332", true);
        assert_eq!(t, FullHouse);
        let t = Hand::strongest_type("2333A", true);
        assert_eq!(t, ThreeOfKind);
        let t = Hand::strongest_type("23432", true);
        assert_eq!(t, TwoPair);
        let t = Hand::strongest_type("A23A4", true);
        assert_eq!(t, OnePair);
        let t = Hand::strongest_type("23456", true);
        assert_eq!(t, HighCard);
    }

    #[test]
    fn sorts() {
        let threes = Hand::strongest_type("33332", true);
        assert_eq!(threes, FourOfKind);
        let aas = Hand::strongest_type("2AAAA", true);
        assert_eq!(aas, FourOfKind);

        let mut hands = vec![Hand::new("33332 1", true), Hand::new("2AAAA 2", true)];
        sort(&mut hands);
        assert_eq!(hands[0].bid, 2)
    }

    #[test]
    fn part_two_jokers() {
        let a = Hand::strongest_type("JJJJJ", false);
        assert_eq!(a, FiveOfKind);
    }
    #[test]
    fn part_two_expected() {
        let a = Hand::strongest_type("QJJQ2", false);
        assert_eq!(a, FourOfKind);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
