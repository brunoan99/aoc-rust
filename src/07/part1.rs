use std::collections::HashMap;

pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 07 - Part 01 - {output}");
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord, Hash)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl From<char> for Card {
  fn from(value: char) -> Self {
    match value {
      '2' => Card::Two,
      '3' => Card::Three,
      '4' => Card::Four,
      '5' => Card::Five,
      '6' => Card::Six,
      '7' => Card::Seven,
      '8' => Card::Eight,
      '9' => Card::Nine,
      'T' => Card::Ten,
      'J' => Card::Jack,
      'Q' => Card::Queen,
      'K' => Card::King,
      'A' => Card::Ace,
      _ => panic!("Unknow card"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
  HighCard,
  Pair,
  TwoPairs,
  ThreeOfKind,
  FullHouse,
  FourOfKind,
  FiveOfKind,
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
  cards: [Card; 5],
  bid: usize,
  strength: HandStrength,
}

impl From<&str> for Hand {
  fn from(value: &str) -> Self {
    let (cards, bid) = value.trim().split_once(" ").unwrap();
    let cards: [Card; 5] = cards
      .trim()
      .chars()
      .into_iter()
      .map(|c| Card::from(c))
      .collect::<Vec<Card>>()
      .try_into()
      .unwrap();

    let bid = bid.parse::<usize>().unwrap();

    let mut map: HashMap<Card, usize> = HashMap::with_capacity(5);

    for card in cards.iter() {
      match map.get_mut(card) {
        Some(value) => *value = *value + 1,
        None => {
          map.insert(card.clone(), 1);
        }
      };
    }
    let mut tally: Vec<usize> = map.values().copied().collect();
    tally.sort_by(|x, y| y.cmp(x));

    let strength = match tally[..] {
      [1, 1, 1, 1, 1] => HandStrength::HighCard,
      [2, 1, 1, 1] => HandStrength::Pair,
      [3, 1, 1] => HandStrength::ThreeOfKind,
      [2, 2, 1] => HandStrength::TwoPairs,
      [3, 2] => HandStrength::FullHouse,
      [4, 1] => HandStrength::FourOfKind,
      [5] => HandStrength::FiveOfKind,
      _ => panic!("Unknow Strength"),
    };

    Hand {
      cards,
      bid,
      strength,
    }
  }
}

struct Play {
  plays: Vec<Hand>,
}

impl From<&str> for Play {
  fn from(value: &str) -> Self {
    let plays: Vec<Hand> = value.trim().lines().map(|line| Hand::from(line)).collect();
    Play { plays }
  }
}

impl Play {
  pub fn sort(&mut self) {
    self
      .plays
      .sort_unstable_by_key(|hand| (hand.strength.clone(), hand.cards.clone()))
  }
}

fn process(input: &str) -> usize {
  let mut play = Play::from(input);
  play.sort();
  play
    .plays
    .iter()
    .enumerate()
    .map(|(index, play)| (index + 1) * play.bid)
    .fold(0, |acc, score| acc + score)
}

#[cfg(test)]
mod test_day07_part01_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    assert_eq!(process(input), 6440);
  }
}
