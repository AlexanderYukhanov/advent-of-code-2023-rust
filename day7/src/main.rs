use std::{cmp::Ordering, collections::HashMap, fs};

const CARDS_ORDER: &str = "AKQJT98765432";

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
}

impl Hand {
    // Calculates the rank of a hand: five of a kind = 0, ..., high card = 6
    fn rank(&self, joker: char) -> u8 {
        let jokers = self.cards.chars().filter(|c| *c == joker).count();
        let mut kinds: HashMap<char, usize> = HashMap::new();
        self.cards
            .chars()
            .filter(|c| *c != joker)
            .fold(&mut kinds, |acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });
        let len = kinds.len();
        let mut longest = kinds.values().max().and_then(|a| Some(*a)).unwrap_or(0);
        longest += jokers;
        match (len, longest) {
            (0, _) => 0, // all jokers == five of a kind
            (1, _) => 0, // five of a kind
            (2, 4) => 1, // four of a kind
            (2, _) => 2, // full house
            (3, 3) => 3, // three of a kind
            (3, _) => 4, // two pairs
            (4, _) => 5, // a pair
            _ => 6,      // high card
        }
    }

    // Calculates a weight of a hand based on individual cards:
    // such as the stronger card has a less weight
    fn weight(&self, joker: char, ordering: &str) -> usize {
        return self
            .cards
            .chars()
            .map(|c| {
                if c == joker {
                    ordering.len()
                } else {
                    ordering.find(c).unwrap()
                }
            })
            .fold(0, |a, v| a * (ordering.len() + 1) + v);
    }
}

fn comparator(joker: char) -> impl for<'a, 'b> Fn(&'a Hand, &'b Hand) -> std::cmp::Ordering {
    return move |lhs: &Hand, rhs: &Hand| -> Ordering {
        return lhs.rank(joker).cmp(&rhs.rank(joker)).then_with(|| {
            lhs.weight(joker, CARDS_ORDER)
                .cmp(&rhs.weight(joker, CARDS_ORDER))
        });
    };
}

fn accumulate_bids(hands: &Vec<Hand>) -> usize {
    return hands
        .iter()
        .enumerate()
        .fold(0, |acc, (ind, hand)| acc + (hands.len() - ind) * hand.bid);
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
    hands.sort_by(comparator(' '));
    println!("Part 1: {}", accumulate_bids(&hands));
    hands.sort_by(comparator('J'));
    println!("Part 2: {}", accumulate_bids(&hands));
}
