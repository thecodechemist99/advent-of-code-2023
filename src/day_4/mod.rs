use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn count_wins(&self) -> usize {
        self.numbers.intersection(&self.winning).count()
    }

    fn get_points(&self) -> u32 {
        if self.count_wins() > 0 {
            2_u32.pow(self.count_wins() as u32 - 1)
        } else {
            0
        }
    }
}

// /// Returns copies of the won cards for a specified card
// fn win_cards(stack: &Vec<Card>, card: &Card) -> Vec<Card> {
//     let wins = card.count_wins();
//     (1..=wins as usize)
//         .filter_map(|i| stack.clone().into_iter().find(|c| c.id == card.id + i))
//         .collect()
// }

/// Adds copies of the won cards for a specified card and returns amount of current card
fn win_cards(stack: &mut HashMap<usize, usize>, card: &Card) -> usize {
    let wins = card.count_wins();
    let current_cards = stack.get(&card.id).unwrap_or(&0) + 1;
    for i in 1..=wins {
        *stack.entry(card.id + i).or_insert(0) += current_cards;
    }
    current_cards
}

fn parse_line(input: &str) -> Card {
    let (id, card) = input.split_once(": ").unwrap();
    let (winning, nums) = card.split_once(" | ").unwrap();
    let id = id.split_once(" ").unwrap().1.trim().parse().unwrap();

    Card {
        id,
        winning: winning
            .split(" ")
            .filter_map(|num| num.trim().parse::<u32>().ok())
            .collect(),
        numbers: nums
            .split(" ")
            .filter_map(|num| num.trim().parse::<u32>().ok())
            .collect(),
    }
}

pub fn day_4(input: String) -> (u32, u32) {
    let cards: Vec<Card> = input.lines().map(|line| parse_line(line)).collect();

    let points = cards.iter().map(|card| card.get_points()).sum();

    // Run through stack of cards and produce new ones according to the rules until no winning cards are left
    // let mut count = cards.len() as u32;
    // let mut won: Vec<Card> = cards
    //     .iter()
    //     .flat_map(|card| win_cards(&cards, card))
    //     .collect();

    // while won.len() > 0 {
    //     println!("{:?}, {:?}", count, won.len());
    //     count += won.len() as u32;
    //     won = won.iter().flat_map(|card| win_cards(&won, &card)).collect();
    // }

    // Run through stack of cards and produce new ones according to the rules until no winning cards are left
    // Use HashMap for performance
    // Answer 4744341 is too low
    let mut stack: HashMap<usize, usize> = HashMap::new();
    let count = cards
        .iter()
        .map(|card| win_cards(&mut stack, &card) as u32)
        .sum::<u32>();

    (points, count)
}

#[cfg(test)]
mod tests {
    use crate::day_4::{day_4, parse_line};
    use crate::read_input;

    #[test]
    fn part_one_example_test() {
        let input = read_input("src/day_4/example.txt");

        assert_eq!(13, day_4(input).0);
    }

    #[test]
    fn part_two_example_test() {
        let input = read_input("src/day_4/example.txt");

        assert_eq!(30, day_4(input).1);
    }
}
