use std::collections::HashMap;

pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 07 - Part 02 - {output}");
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord, Hash)]
pub enum Card {
  Joker,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Queen,
  King,
  Ace,
}

impl From<char> for Card {
  fn from(value: char) -> Self {
    match value {
      'J' => Card::Joker,
      '2' => Card::Two,
      '3' => Card::Three,
      '4' => Card::Four,
      '5' => Card::Five,
      '6' => Card::Six,
      '7' => Card::Seven,
      '8' => Card::Eight,
      '9' => Card::Nine,
      'T' => Card::Ten,
      'Q' => Card::Queen,
      'K' => Card::King,
      'A' => Card::Ace,
      _ => panic!("Unknow card"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
struct Hand {
  cards: [Card; 5],
  bid: usize,
}

impl From<&str> for Hand {
  fn from(value: &str) -> Self {
    let (cards, score) = value.trim().split_once(" ").unwrap();
    let cards: [Card; 5] = cards
      .trim()
      .chars()
      .into_iter()
      .map(|c| Card::from(c))
      .collect::<Vec<Card>>()
      .try_into()
      .unwrap();

    let score = score.parse::<usize>().unwrap();

    Hand { cards, bid: score }
  }
}

struct HandMap {
  map: HashMap<Card, usize>,
}

impl From<&Hand> for HandMap {
  fn from(value: &Hand) -> Self {
    let mut map: HashMap<Card, usize> = HashMap::with_capacity(5);

    for card in value.cards.iter() {
      // ignore jokers
      if card == &Card::Joker {
        continue;
      }
      let new_value = match map.get(card) {
        Some(value) => value + 1,
        None => 1,
      };
      map.insert(card.clone(), new_value);
    }

    HandMap { map }
  }
}

impl Hand {
  fn count_jokers(&self) -> usize {
    self
      .cards
      .iter()
      .filter(|&card| card == &Card::Joker)
      .collect::<Vec<_>>()
      .len()
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
struct HandEvaluated {
  cards: [Card; 5],
  bid: usize,
  strength: HandStrength,
}

impl From<Hand> for HandEvaluated {
  fn from(value: Hand) -> Self {
    let map = HandMap::from(&value).map;
    let mut tally: Vec<usize> = map.values().copied().collect();
    tally.sort_by(|x, y| y.cmp(x));

    let jokers = value.count_jokers();
    if jokers == 5 {
      tally.push(jokers);
    } else {
      tally[0] += jokers;
    }
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

    HandEvaluated {
      cards: value.cards,
      bid: value.bid,
      strength,
    }
  }
}

#[derive(Debug)]
struct Play {
  plays: Vec<Hand>,
}

impl From<&str> for Play {
  fn from(value: &str) -> Self {
    let plays: Vec<Hand> = value.trim().lines().map(|line| Hand::from(line)).collect();
    Play { plays }
  }
}

#[derive(Debug)]
struct PlayComputed {
  plays: Vec<HandEvaluated>,
}

impl From<Play> for PlayComputed {
  fn from(value: Play) -> Self {
    let plays = value
      .plays
      .into_iter()
      .map(|hand| HandEvaluated::from(hand))
      .collect();
    Self { plays }
  }
}

impl PlayComputed {
  pub fn sort(&mut self) {
    self
      .plays
      .sort_unstable_by_key(|hand| (hand.strength.clone(), hand.cards.clone()))
  }
}

fn process(input: &str) -> usize {
  let play = Play::from(input);
  let mut comp_play = PlayComputed::from(play);
  comp_play.sort();
  comp_play
    .plays
    .iter()
    .enumerate()
    .map(|(index, play)| (index + 1) * play.bid)
    .fold(0, |acc, score| acc + score)
}

#[cfg(test)]
mod test_day07_part02_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";
    assert_eq!(process(input), 5905);
  }
}
// 254523603
// 254837398
