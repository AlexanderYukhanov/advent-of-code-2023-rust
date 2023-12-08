use std::{cmp::Ordering, collections::HashMap, fs};

const CARDS_ORDER: &str = "AKQJT98765432";
const CARDS_ORDER_2: &str = "AKQT98765432J";

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: String,
    bid: usize,
}

impl Hand {
    fn weight(&self) -> usize {
        return self
            .cards
            .chars()
            .map(|c| CARDS_ORDER.find(c).unwrap())
            .fold(0, |a, v| a * 16 + v);
    }
    fn weight2(&self) -> usize {
        return self
            .cards
            .chars()
            .map(|c| CARDS_ORDER_2.find(c).unwrap())
            .fold(0, |a, v| a * 16 + v);
    }
    fn rank(&self) -> u8 {
        let mut kinds: HashMap<char, usize> = HashMap::new();
        self.cards.chars().fold(&mut kinds, |acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        if kinds.len() == 1 {
            return 0;
        }
        if kinds.len() == 2 && *kinds.values().max().unwrap() == 4 {
            return 1;
        }
        if kinds.len() == 2 {
            return 2;
        }
        if kinds.len() == 3 && *kinds.values().max().unwrap() == 3 {
            return 3;
        }
        if kinds.len() == 3 {
            return 4;
        }
        if kinds.len() == 4 {
            return 5;
        }
        return 6;
    }

    fn rank2(&self) -> u8 {
        let mut kinds: HashMap<char, usize> = HashMap::new();
        let jokers = self.cards.chars().filter(|c| *c == 'J').count();
        self.cards.chars().filter(|c| *c != 'J').fold(&mut kinds, |acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let len = kinds.len();
        let mut longest = kinds.values().max().and_then(|a| Some(*a)).unwrap_or(0);
        if len == 0 {
            return 0;
        }
        longest += jokers;
        if len == 1 {
            return 0;
        }
        if len == 2 && longest == 4 {
            return 1;
        }
        if len == 2 {
            return 2;
        }
        if len == 3 && longest == 3 {
            return 3;
        }
        if len == 3 {
            return 4;
        }
        if len == 4 {
            return 5;
        }
        return 6;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self
            .rank()
            .cmp(&other.rank())
            .then_with(|| self.weight().cmp(&other.weight()));
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2cmp(lhs: &Hand, rhs: &Hand) -> Ordering {
    return lhs
            .rank2()
            .cmp(&rhs.rank2())
            .then_with(|| lhs.weight2().cmp(&rhs.weight2()));
}

fn main() {
    let mut hands: Vec<Hand> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|l| l.trim().split(" "))
        .map(|mut s| Hand {
            cards: s.next().unwrap().to_owned(),
            bid: s.next().unwrap().parse().unwrap(),
        })
        .collect();
    hands.sort();
    let part1 = hands.iter().enumerate().fold(0, |acc, h| {
        let (ind, &ref hand) = h;
        return acc + (hands.len() - ind) * hand.bid
    });
    hands.sort_by(part2cmp);
    for h in &hands {
        println!("{} {} {}", h.cards, h.rank(), h.weight());
    }
    let part2 = hands.iter().enumerate().fold(0, |acc, h| {
        let (ind, &ref hand) = h;
        return acc + (hands.len() - ind) * hand.bid
    });
    println!("{}", part1);
    println!("{}", part2);
}
