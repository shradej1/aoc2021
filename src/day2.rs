//!--- Day 2: Dive! ---
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1,
//! down 2, or up 3:
//!
//!     forward X increases the horizontal position by X units.
//!     down X increases the depth by X units.
//!     up X decreases the depth by X units.
//!
//! Note that since you're on a submarine, down and up affect your depth, and so
//! they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//!
//! Your horizontal position and depth both start at 0. The steps above would
//! then modify them as follows:
//!
//!     forward 5 adds 5 to your horizontal position, a total of 5.
//!     down 5 adds 5 to your depth, resulting in a value of 5.
//!     forward 8 adds 8 to your horizontal position, a total of 13.
//!     up 3 decreases your depth by 3, resulting in a value of 2.
//!     down 8 adds 8 to your depth, resulting in a value of 10.
//!     forward 2 adds 2 to your horizontal position, a total of 15.
//!
//! After following these instructions, you would have a horizontal position of
//! 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following
//! the planned course. What do you get if you multiply your final horizontal
//! position by your final depth?
//!
//! --- Part Two ---
//!
//! Based on your calculations, the planned course doesn't seem to make any
//! sense. You find the submarine manual and discover that the process is
//! actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a
//! third value, aim, which also starts at 0. The commands also mean something
//! entirely different than you first thought:
//!
//!     down X increases your aim by X units.
//!     up X decreases your aim by X units.
//!     forward X does two things:
//!         It increases your horizontal position by X units.
//!         It increases your depth by your aim multiplied by X.
//!
//! Again note that since you're on a submarine, down and up do the opposite of
//! what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//!     forward 5 adds 5 to your horizontal position, a total of 5. Because your
//! aim is 0, your depth does not change.     down 5 adds 5 to your aim,
//! resulting in a value of 5.     forward 8 adds 8 to your horizontal position,
//! a total of 13. Because your aim is 5, your depth increases by 8*5=40.     up
//! 3 decreases your aim by 3, resulting in a value of 2.     down 8 adds 8 to
//! your aim, resulting in a value of 10.     forward 2 adds 2 to your
//! horizontal position, a total of 15. Because your aim is 10, your depth
//! increases by 2*10=20 to a total of 60.
//!
//! After following these new instructions, you would have a horizontal position
//! of 15 and a depth of 60. (Multiplying these produces 900.)
//!
//! Using this new interpretation of the commands, calculate the horizontal
//! position and depth you would have after following the planned course. What
//! do you get if you multiply your final horizontal position by your final
//! depth?

use std::str::FromStr;

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

/// Parses the puzzle input into a Vec of directions
fn parse_input(input: &str) -> Vec<Direction> {
    let mut dirs = Vec::new();
    for line in input.lines() {
        let toks: Vec<_> = line.split_ascii_whitespace().collect();
        match &toks[..] {
            &[dir, amount] => {
                let amount = i32::from_str(amount).unwrap();
                let dir = match dir {
                    "forward" => Direction::Forward(amount),
                    "up" => Direction::Up(amount),
                    "down" => Direction::Down(amount),
                    _ => panic!("Unexpected direction: {}", dir),
                };
                dirs.push(dir);
            }
            _ => {}
        }
    }

    dirs
}

fn compute_position(directions: &[Direction]) -> Position {
    directions.iter().fold(Position::default(), |mut pos, dir| {
        match dir {
            Direction::Forward(a) => pos.horizontal += a,
            Direction::Up(a) => pos.depth -= a,
            Direction::Down(a) => pos.depth += a,
        }
        pos
    })
}

#[derive(Debug, Default)]
struct PositionWithAim {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn compute_position_with_aim(directions: &[Direction]) -> PositionWithAim {
    directions
        .iter()
        .fold(PositionWithAim::default(), |mut pos, dir| {
            match dir {
                Direction::Forward(a) => {
                    pos.horizontal += a;
                    pos.depth += a * pos.aim;
                }
                Direction::Up(a) => pos.aim -= a,
                Direction::Down(a) => pos.aim += a,
            }
            pos
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_puzzle_input;

    #[test]
    fn solve_part_1() {
        let input = read_puzzle_input("day2-puzzle-input.txt");
        let input = parse_input(&input);
        let final_position = compute_position(&input);
        let answer = final_position.depth * final_position.horizontal;
        assert_eq!(1989265, answer);
    }

    #[test]
    fn solve_part_2() {
        let input = read_puzzle_input("day2-puzzle-input.txt");
        let input = parse_input(&input);
        let final_position = compute_position_with_aim(&input);
        let answer = final_position.depth * final_position.horizontal;
        assert_eq!(2089174012, answer);
    }
}
