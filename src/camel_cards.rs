use crate::solver::{MultiSolver, Solver};
use anyhow::{anyhow, Error, Result};
use std::{cmp::Reverse, fs::read_to_string, path::PathBuf};

///     --- Day 7: Camel Cards ---
///
/// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
///
/// "Did you bring the parts?"
///
/// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.
///
/// "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.
///
/// "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.
///
/// After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.
///
/// You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.
///
/// Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.
///
/// In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
///
/// Every hand is exactly one type. From strongest to weakest, they are:
///
///     - Five of a kind, where all five cards have the same label: AAAAA
///     - Four of a kind, where four cards have the same label and one card has a different label: AA8AA
///     - Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
///     - Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
///     - Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
///     - One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
///     - High card, where all cards' labels are distinct: 23456
///
/// Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.
///
/// If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.
///
/// So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
///
/// To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
/// This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
///
/// So, the first step is to put the hands in order of strength:
///
///     - 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
///     - KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
///     - T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
///
/// Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.
///
/// Find the rank of every hand in your set. What are the total winnings?
///
///     --- Part Two ---
///
/// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.
///
/// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
///
/// J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
///
/// Now, the above example goes very differently:
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///     - 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
///     - KK677 is now the only two pair, making it the second-weakest hand.
///     - T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
///
/// With the new joker rule, the total winnings in this example are 5905.
///
/// Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
pub struct CamelCards;
pub struct PartOne;
pub struct PartTwo;

impl MultiSolver for CamelCards {
    type PartOne = PartOne;
    type PartTwo = PartTwo;

    fn get_puzzle_title(&self) -> &str {
        "Day 7: Camel Cards"
    }

    fn get_part_one(&self) -> Self::PartOne {
        PartOne
    }

    fn get_part_two(&self) -> Self::PartTwo {
        PartTwo
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::_9),
            '8' => Ok(Card::_8),
            '7' => Ok(Card::_7),
            '6' => Ok(Card::_6),
            '5' => Ok(Card::_5),
            '4' => Ok(Card::_4),
            '3' => Ok(Card::_3),
            '2' => Ok(Card::_2),
            _ => Err(anyhow!("Invalid card: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum CardJokerRule {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    _9 = 9,
    _8 = 8,
    _7 = 7,
    _6 = 6,
    _5 = 5,
    _4 = 4,
    _3 = 3,
    _2 = 2,
    J = 1,
}

impl TryFrom<char> for CardJokerRule {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Error> {
        match c {
            'A' => Ok(CardJokerRule::A),
            'K' => Ok(CardJokerRule::K),
            'Q' => Ok(CardJokerRule::Q),
            'J' => Ok(CardJokerRule::J),
            'T' => Ok(CardJokerRule::T),
            '9' => Ok(CardJokerRule::_9),
            '8' => Ok(CardJokerRule::_8),
            '7' => Ok(CardJokerRule::_7),
            '6' => Ok(CardJokerRule::_6),
            '5' => Ok(CardJokerRule::_5),
            '4' => Ok(CardJokerRule::_4),
            '3' => Ok(CardJokerRule::_3),
            '2' => Ok(CardJokerRule::_2),
            _ => Err(anyhow!("Invalid card: {}", c)),
        }
    }
}

impl From<CardJokerRule> for Card {
    fn from(card: CardJokerRule) -> Self {
        match card {
            CardJokerRule::A => Card::A,
            CardJokerRule::K => Card::K,
            CardJokerRule::Q => Card::Q,
            CardJokerRule::J => Card::J,
            CardJokerRule::T => Card::T,
            CardJokerRule::_9 => Card::_9,
            CardJokerRule::_8 => Card::_8,
            CardJokerRule::_7 => Card::_7,
            CardJokerRule::_6 => Card::_6,
            CardJokerRule::_5 => Card::_5,
            CardJokerRule::_4 => Card::_4,
            CardJokerRule::_3 => Card::_3,
            CardJokerRule::_2 => Card::_2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

trait HandOfCards {
    fn get_type(&self) -> HandType;
}

#[derive(Debug, Clone)]
struct Hand<CardType> {
    pub cards: [CardType; 5],
    pub bid: u64,
}

impl HandOfCards for Hand<Card> {
    fn get_type(&self) -> HandType {
        let mut card_counts = self
            .cards
            .iter()
            .map(|anchor_card| {
                self.cards
                    .iter()
                    .filter(|card| *card == anchor_card)
                    .count()
            })
            .collect::<Vec<usize>>();
        card_counts.sort_by_key(|count| Reverse(*count));

        match card_counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_counts[3] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if card_counts[2] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Invalid card count!"),
        }
    }
}

impl HandOfCards for Hand<CardJokerRule> {
    fn get_type(&self) -> HandType {
        let new_hand: Hand<Card> = if self.cards.iter().any(|card| card == &CardJokerRule::J) {
            let mut sorted_cards = self.cards.clone();
            sorted_cards.sort_by_key(|card| Reverse(*card));
            let mode_card = sorted_cards
                .get(
                    sorted_cards
                        .iter()
                        .enumerate()
                        .max_by_key(|(_, card)| match card {
                            CardJokerRule::J => 0,
                            _ => sorted_cards.iter().filter(|c| c == card).count(),
                        })
                        .map(|(i, _)| i)
                        .unwrap_or(0),
                )
                .unwrap();

            Hand {
                cards: self
                    .cards
                    .iter()
                    .map(|card| {
                        Card::from(match card {
                            CardJokerRule::J => *mode_card,
                            _ => *card,
                        })
                    })
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                bid: self.bid,
            }
        } else {
            Hand {
                cards: self
                    .cards
                    .iter()
                    .map(|card| Card::from(*card))
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                bid: self.bid,
            }
        };

        new_hand.get_type()
    }
}

impl<CardType> Ord for Hand<CardType>
where
    Hand<CardType>: HandOfCards,
    CardType: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        for (card_a, card_b) in self.cards.iter().zip(other.cards.iter()) {
            if card_a != card_b {
                return card_a.cmp(&card_b);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl<CardType> PartialOrd for Hand<CardType>
where
    Hand<CardType>: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<CardType> Eq for Hand<CardType> where Hand<CardType>: Ord {}
impl<CardType> PartialEq for Hand<CardType>
where
    Hand<CardType>: Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl<CardType: TryFrom<char, Error = anyhow::Error> + core::fmt::Debug> TryFrom<&str>
    for Hand<CardType>
{
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        let cards_bid = s
            .split_whitespace()
            .take(2)
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        Ok(Self {
            cards: cards_bid[0]
                .chars()
                .take(5)
                .map(|c| CardType::try_from(c))
                .collect::<Result<Vec<CardType>, _>>()?
                .try_into()
                .unwrap(),
            bid: cards_bid[1].parse::<u64>()?,
        })
    }
}

fn solve<CardType>(data: &str) -> Result<i32>
where
    CardType: TryFrom<char, Error = anyhow::Error> + core::fmt::Debug + Copy + Ord,
    Hand<CardType>: HandOfCards,
{
    let mut hands = data
        .lines()
        .map(|line| Hand::try_from(line))
        .collect::<Result<Vec<Hand<CardType>>, _>>()?;
    hands.sort();
    let mut rank: u64 = 1;
    let mut ranked_hands = hands
        .windows(2)
        .map(|hands| {
            let hand_a = &hands[0];
            let hand_b = &hands[1];
            if hand_a != hand_b {
                rank += 1;
            }
            (hand_a.clone(), rank - 1)
        })
        .collect::<Vec<(Hand<CardType>, u64)>>();
    ranked_hands.push((hands.last().unwrap().clone(), rank));
    let total = ranked_hands
        .iter()
        .map(|(hand, rank)| (hand.bid * rank) as u64)
        .sum::<u64>();
    Ok(total as i32)
}

impl Solver for PartOne {
    fn part_description(&self) -> (u32, &str) {
        (1, "Total winnings")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        let data = read_to_string(filepath)?;
        solve::<Card>(&data)
    }
}

impl Solver for PartTwo {
    fn part_description(&self) -> (u32, &str) {
        (2, "Total winnings")
    }

    fn get_solution(&self, filepath: &PathBuf) -> Result<i32> {
        let data = read_to_string(filepath)?;
        solve::<CardJokerRule>(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let filepath = std::path::PathBuf::from("data/07/input");
        let solver = PartOne;
        let solution = solver.get_solution(&filepath)?;
        assert_eq!(solution, 251121738);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let filepath = std::path::PathBuf::from("data/07/input");
        let solver = PartTwo;
        let solution = solver.get_solution(&filepath)?;
        assert_eq!(solution, 251421071);
        Ok(())
    }
}
