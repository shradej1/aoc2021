//! --- Day 4: Giant Squid ---
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean,
//! already so deep that you can't see any sunlight. What you can see, however,
//! is a giant squid that has attached itself to the outside of your submarine.
//!
//! Maybe it wants to play bingo?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
//! Numbers are chosen at random, and the chosen number is marked on all boards
//! on which it appears. (Numbers may not appear on all boards.) If all numbers
//! in any row or any column of a board are marked, that board wins. (Diagonals
//! don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and
//! the giant squid) pass the time. It automatically generates a random order in
//! which to draw numbers and a random set of boards (your puzzle input). For
//! example:
//!
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//!
//! After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
//! winners, but the boards are marked as follows (shown here adjacent to each
//! other to save space):
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are
//! still no winners:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! Finally, 24 is drawn:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! At this point, the third board wins because it has at least one complete row
//! or column of marked numbers (in this case, the entire top row is marked: 14
//! 21 17 24 4).
//!
//! The score of the winning board can now be calculated. Start by finding the
//! sum of all unmarked numbers on that board; in this case, the sum is 188.
//! Then, multiply that sum by the number that was just called when the board
//! won, 24, to get the final score, 188 * 24 = 4512.
//!
//! To guarantee victory against the giant squid, figure out which board will
//! win first. What will your final score be if you choose that board?

use std::str::FromStr;

struct BingoCard {
    nums: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl BingoCard {
    fn new(nums: [[u8; 5]; 5]) -> Self {
        Self {
            nums,
            marks: [[false; 5]; 5],
        }
    }

    fn mark(&mut self, num: u8) {
        for i in 0..self.nums.len() {
            for j in 0..self.nums[i].len() {
                if self.nums[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        (0..5).any(|i| self.row_is_winner(i) || self.col_is_winner(i))
    }

    fn row_is_winner(&self, row_idx: usize) -> bool {
        (0..5).all(|col_idx| self.marks[row_idx][col_idx])
    }

    fn col_is_winner(&self, col_idx: usize) -> bool {
        (0..5).all(|row_idx| self.marks[row_idx][col_idx])
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.nums.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                if !self.marks[i][j] {
                    sum += *entry as u32;
                }
            }
        }

        sum
    }
}

fn parse_line(x: &str) -> [u8; 5] {
    let v: Vec<_> = x
        .split_ascii_whitespace()
        .map(|c| u8::from_str(c).unwrap())
        .collect();

    [v[0], v[1], v[2], v[3], v[4]]
}

/// Parses the puzzle input into the random number draws and cards.
fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoCard>) {
    let mut lines = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .peekable();

    let rand_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| u8::from_str(c).unwrap())
        .collect();

    let mut cards = Vec::new();
    while lines.peek().is_some() {
        let mut card = [[0_u8; 5]; 5];
        card[0] = parse_line(lines.next().unwrap());
        card[1] = parse_line(lines.next().unwrap());
        card[2] = parse_line(lines.next().unwrap());
        card[3] = parse_line(lines.next().unwrap());
        card[4] = parse_line(lines.next().unwrap());
        cards.push(BingoCard::new(card))
    }

    (rand_numbers, cards)
}

fn mark_cards(cards: &mut Vec<BingoCard>, num: u8) {
    cards.iter_mut().for_each(|c| c.mark(num));
}

/// Using `nums`, determines which bingo cards win, and returns them in the
/// order in which they win.  In other words, the first entry in the returned
/// vector is the earliest card to win under the given `nums`.  Each card is
/// mapped to the number that made it a winner.
fn sort_into_winners(nums: Vec<u8>, cards: Vec<BingoCard>) -> Vec<(BingoCard, u8)> {
    nums.into_iter()
        .fold((cards, Vec::new()), |(mut cards, mut winners), num| {
            mark_cards(&mut cards, num);

            // partition cards into winners and non-winners
            let (new_winners, non_winners): (Vec<_>, Vec<_>) =
                cards.into_iter().partition(|c| c.is_winner());

            // map winners to (winning_number, card), and extend winners with mapped
            // new_winners.
            winners.extend(new_winners.into_iter().map(|w| (w, num)));
            (non_winners, winners)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_puzzle_input;

    /// ## Numbers
    /// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
    ///
    /// ## Bingo Card
    /// 14 21 17 24  4
    /// 10 16 15  9 19
    /// 18  8 23 26 20
    /// 22 11 13  6  5
    ///  2  0 12  3  7
    ///
    /// Should win after 24 is called
    #[test]
    fn test_board_3() {
        let input_str = r"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
        
         14 21 17 24  4
         10 16 15  9 19
         18  8 23 26 20
         22 11 13  6  5
          2  0 12  3  7";

        let (nums, mut cards) = parse_input(input_str);
        assert_eq!(nums.len(), 27);
        assert_eq!(cards.len(), 1);

        let mut nums = nums.into_iter();
        mark_cards(&mut cards, nums.next().unwrap());

        assert!(cards[0].marks[4][4]);
        assert!(!cards[0].marks[0][0]);

        let mut last_called = 0;
        while !cards[0].is_winner() {
            last_called = nums.next().unwrap();
            mark_cards(&mut cards, last_called);
        }

        assert_eq!(last_called, 24);
        assert_eq!(cards[0].sum_unmarked(), 188);
    }

    /// Compute score of first board to win.
    #[test]
    fn solve_part_1() {
        let (nums, cards) = parse_input(&read_puzzle_input("day4-puzzle-input.txt"));
        let winners = sort_into_winners(nums, cards);
        let (first_card_to_win, winning_number) = winners.first().unwrap();
        assert_eq!(
            *winning_number as u32 * first_card_to_win.sum_unmarked(),
            54275
        );
    }

    /// Compute score of last board to win.
    #[test]
    fn solve_part_2() {
        let (nums, cards) = parse_input(&read_puzzle_input("day4-puzzle-input.txt"));
        let winners = sort_into_winners(nums, cards);
        let (last_card_to_win, winning_number) = winners.last().unwrap();
        assert_eq!(
            *winning_number as u32 * last_card_to_win.sum_unmarked(),
            13158
        );
    }
}
