//!--- Day 8: Seven Segment Search ---
//!
//! You barely reach the safety of the cave when the whale smashes into the cave
//! mouth, collapsing it. Sensors indicate another exit to this cave at a much
//! greater depth, so you have no choice but to press on.
//!
//! As your submarine slowly makes its way through the cave system, you notice
//! that the four-digit seven-segment displays in your submarine are
//! malfunctioning; they must have been damaged during the escape. You'll be in
//! a lot of trouble without them, so you'd better figure out what's wrong.
//!
//! Each digit of a seven-segment display is rendered by turning on or off any
//! of seven segments named a through g:
//!
//!   0:      1:      2:      3:      4:
//!  aaaa    ....    aaaa    aaaa    ....
//! b    c  .    c  .    c  .    c  b    c
//! b    c  .    c  .    c  .    c  b    c
//!  ....    ....    dddd    dddd    dddd
//! e    f  .    f  e    .  .    f  .    f
//! e    f  .    f  e    .  .    f  .    f
//!  gggg    ....    gggg    gggg    ....
//!
//!   5:      6:      7:      8:      9:
//!  aaaa    aaaa    aaaa    aaaa    aaaa
//! b    .  b    .  .    c  b    c  b    c
//! b    .  b    .  .    c  b    c  b    c
//!  dddd    dddd    ....    dddd    dddd
//! .    f  e    f  .    f  e    f  .    f
//! .    f  e    f  .    f  e    f  .    f
//!  gggg    gggg    ....    gggg    gggg
//!
//! So, to render a 1, only segments c and f would be turned on; the rest would
//! be off. To render a 7, only segments a, c, and f would be turned on.
//!
//! The problem is that the signals which control the segments have been mixed
//! up on each display. The submarine is still trying to display numbers by
//! producing output on signal wires a through g, but those wires are connected
//! to segments randomly. Worse, the wire/segment connections are mixed up
//! separately for each four-digit display! (All of the digits within a display
//! use the same connections, though.)
//!
//! So, you might know that only signal wires b and g are turned on, but that
//! doesn't mean segments b and g are turned on: the only digit that uses two
//! segments is 1, so it must mean segments c and f are meant to be on. With
//! just that information, you still can't tell which wire (b/g) goes to which
//! segment (c/f). For that, you'll need to collect more information.
//!
//! For each display, you watch the changing signals for a while, make a note of
//! all ten unique signal patterns you see, and then write down a single four
//! digit output value (your puzzle input). Using the signal patterns, you
//! should be able to work out which pattern corresponds to which digit.
//!
//! For example, here is what you might see in a single entry in your notes:
//!
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//!
//! (The entry is wrapped here to two lines so it fits; in your notes, it will
//! all be on a single line.)
//!
//! Each entry consists of ten unique signal patterns, a | delimiter, and
//! finally the four digit output value. Within an entry, the same wire/segment
//! connections are used (but you don't know what the connections actually are).
//! The unique signal patterns correspond to the ten different ways the
//! submarine tries to render a digit using the current wire/segment
//! connections. Because 7 is the only digit that uses three segments, dab in
//! the above example means that to render a 7, signal lines d, a, and b are on.
//! Because 4 is the only digit that uses four segments, eafb means that to
//! render a 4, signal lines e, a, f, and b are on.
//!
//! Using this information, you should be able to work out which combination of
//! signal wires corresponds to each of the ten digits. Then, you can decode the
//! four digit output value. Unfortunately, in the above example, all of the
//! digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and
//! are more difficult to deduce.
//!
//! For now, focus on the easy digits. Consider this larger example:
//!
//! be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//! fdgacbe cefdb cefbgd gcbe
//! edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//! fcgedb cgb dgebacf gc
//! fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//! cg cg fdcagb cbg
//! fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//! efabcd cedba gadfec cb
//! aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//! gecf egdcabf bgf bfgea
//! fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//! gebdcfa ecba ca fadegcb
//! dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//! cefg dcbef fcge gbcadfe
//! bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//! ed bcgafe cdgba cbgef
//! egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//! gbdfcae bgc cg cgb
//! gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//! fgae cfgab fg bagce
//!
//! Because the digits 1, 4, 7, and 8 each use a unique number of segments, you
//! should be able to tell which combinations of signals correspond to those
//! digits. Counting only digits in the output values (the part after | on each
//! line), in the above example, there are 26 instances of digits that use a
//! unique number of segments (highlighted above).
//!
//! In the output values, how many times do digits 1, 4, 7, or 8 appear?
//!
//! Your puzzle answer was 383.
//! --- Part Two ---
//!
//! Through a little deduction, you should now be able to determine the
//! remaining digits. Consider again the first example above:
//!
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//!
//! After some careful analysis, the mapping between signal wires and segments
//! only make sense in the following configuration:
//!
//!  dddd
//! e    a
//! e    a
//!  ffff
//! g    b
//! g    b
//!  cccc
//!
//! So, the unique signal patterns would correspond to the following digits:
//!
//!     acedgfb: 8
//!     cdfbe: 5
//!     gcdfa: 2
//!     fbcad: 3
//!     dab: 7
//!     cefabd: 9
//!     cdfgeb: 6
//!     eafb: 4
//!     cagedb: 0
//!     ab: 1
//!
//! Then, the four digits of the output value can be decoded:
//!
//!     cdfeb: 5
//!     fcadb: 3
//!     cdfeb: 5
//!     cdbaf: 3
//!
//! Therefore, the output value for this entry is 5353.
//!
//! Following this same process for each entry in the second, larger example
//! above, the output value of each entry can be determined:
//!
//!     fdgacbe cefdb cefbgd gcbe: 8394
//!     fcgedb cgb dgebacf gc: 9781
//!     cg cg fdcagb cbg: 1197
//!     efabcd cedba gadfec cb: 9361
//!     gecf egdcabf bgf bfgea: 4873
//!     gebdcfa ecba ca fadegcb: 8418
//!     cefg dcbef fcge gbcadfe: 4548
//!     ed bcgafe cdgba cbgef: 1625
//!     gbdfcae bgc cg cgb: 8717
//!     fgae cfgab fg bagce: 4315
//!
//! Adding all of the output values in this larger example produces 61229.
//!
//! For each entry, determine all of the wire/segment connections and decode the
//! four-digit output values. What do you get if you add up all of the output
//! values?

use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|s| s.split_once('|').unwrap())
        .map(|(first, second)| {
            (
                first.split_ascii_whitespace().map(|s| s.trim()).collect(),
                second.split_ascii_whitespace().map(|s| s.trim()).collect(),
            )
        })
        .collect()
}

fn build_decoder_by_intersecting(patterns: &[&str]) -> SevenSegmentDisplayEncoding {
    let c = Vec::<HashSet<&str>>::new();
    todo!()
}

/// Each pattern can be mapped to possible digits by considering the number of
/// lit segments.
fn build_decoder(patterns: &[&str]) -> SevenSegmentDisplayEncoding {
    // 1 gives us the c and f segments
    let c_and_f = patterns.iter().filter(|p| p.len() == 2).assert_unique();
    let c_and_f = c_and_f.chars().collect::<Vec<_>>();
    assert_eq!(c_and_f.len(), 2);

    let mut scrambled_to_real = BTreeMap::new();

    // 7 gives us the a segment as the character that is not also in the
    // representation of 1.
    let a = patterns
        .iter()
        .filter(|p| p.len() == 3)
        .assert_unique()
        .chars()
        .filter(|c| !c_and_f.contains(c))
        .assert_unique();
    assert!(scrambled_to_real.insert(a, 'a').is_none());

    // 4 gives us the b and d segments
    let b_and_d = patterns
        .iter()
        .filter(|p| p.len() == 4)
        .assert_unique()
        .chars()
        .filter(|c| !c_and_f.contains(c))
        .collect::<Vec<_>>();
    assert_eq!(b_and_d.len(), 2);

    // now we have enough information to figure out which signal is 3
    // 3 contains a, both c and f, exactly one of b or d, and g.
    let three = patterns
        .iter()
        .filter(|p| {
            p.len() == 5
                && p.contains(a)
                && p.contains(c_and_f[0])
                && p.contains(c_and_f[1])
                && (p.contains(b_and_d[0]) || p.contains(b_and_d[1]))
        })
        .assert_unique();

    // d is the segment that appears in 3
    let (d, b) = if three.contains(b_and_d[0]) {
        (b_and_d[0], b_and_d[1])
    } else {
        (b_and_d[1], b_and_d[0])
    };
    assert!(scrambled_to_real.insert(b, 'b').is_none());
    assert!(scrambled_to_real.insert(d, 'd').is_none());

    // g is the segment that appears in 3 that is otherwise unaccounted for
    let g = three
        .chars()
        .filter(|c| *c != a && *c != d && *c != c_and_f[0] && *c != c_and_f[1])
        .assert_unique();
    assert!(scrambled_to_real.insert(g, 'g').is_none());

    // // 9 contains all of 3's segments, plus b.
    // let nine = patterns
    //     .iter()
    //     .filter(|p| p.len() == 6 && three.chars().all(|c| p.contains(c)))
    //     .assert_unique();
    //
    // let b = nine.chars().filter(|c| !three.contains(c)).assert_unique();

    // 5 contains a, b, d, c/f, and g, resolving the c/f ambiguity.
    let five = patterns
        .iter()
        .filter(|p| {
            p.len() == 5
                && p.contains(a)
                && p.contains(b)
                && p.contains(d)
                && p.contains(g)
                && (p.contains(c_and_f[0]) || p.contains(c_and_f[1]))
        })
        .assert_unique();

    // how to figure out 2 vs 5 using what we know?  we know a, b, d, g and c/f

    let (c, f) = if five.contains(c_and_f[0]) {
        (c_and_f[1], c_and_f[0])
    } else {
        (c_and_f[0], c_and_f[1])
    };

    assert!(scrambled_to_real.insert(c, 'c').is_none());
    assert!(scrambled_to_real.insert(f, 'f').is_none());

    let e = "abcdefg"
        .chars()
        .filter(|ch| *ch != a && *ch != b && *ch != c && *ch != d && *ch != f && *ch != g)
        .assert_unique();

    SevenSegmentDisplayEncoding::new(a, b, c, d, e, f, g)
}

#[derive(Debug)]
struct SevenSegmentDisplayEncoding {
    map: BTreeMap<char, char>,
}

impl SevenSegmentDisplayEncoding {
    fn new(a: char, b: char, c: char, d: char, e: char, f: char, g: char) -> Self {
        assert_eq!(
            [a, b, c, d, e, f, g].iter().collect::<HashSet<_>>().len(),
            7
        );
        let mut map = BTreeMap::new();
        assert!(map.insert(a, 'a').is_none());
        assert!(map.insert(b, 'b').is_none());
        assert!(map.insert(c, 'c').is_none());
        assert!(map.insert(d, 'd').is_none());
        assert!(map.insert(e, 'e').is_none());
        assert!(map.insert(f, 'f').is_none());
        assert!(map.insert(g, 'g').is_none());
        Self { map }
    }

    fn decode(&self, input: &str) -> i32 {
        let mut on = [0; 7];
        input
            .chars()
            .for_each(|c| on[self.map[&c] as usize - 'a' as usize] = 1);
        match on {
            //a,b, c, d, e, f, g,
            [1, 1, 1, 0, 1, 1, 1] => 0,
            [0, 0, 1, 0, 0, 1, 0] => 1,
            [1, 0, 1, 1, 1, 0, 1] => 2,
            [1, 0, 1, 1, 0, 1, 1] => 3,
            [0, 1, 1, 1, 0, 1, 0] => 4,
            [1, 1, 0, 1, 0, 1, 1] => 5,
            [1, 1, 0, 1, 1, 1, 1] => 6,
            [1, 0, 1, 0, 0, 1, 0] => 7,
            [1, 1, 1, 1, 1, 1, 1] => 8,
            [1, 1, 1, 1, 0, 1, 1] => 9,
            _ => panic!("Unable to decode {}", input),
        }
    }

    fn decode_slice(&self, input: &[&str]) -> i32 {
        let mut val_as_string = String::with_capacity(input.len());

        for s in input {
            let mut on = [0; 7];
            s.chars()
                .for_each(|c| on[self.map[&c] as usize - 'a' as usize] = 1);
            let digit = match on {
                //a,b, c, d, e, f, g,
                [1, 1, 1, 0, 1, 1, 1] => 0,
                [0, 0, 1, 0, 0, 1, 0] => 1,
                [1, 0, 1, 1, 1, 0, 1] => 2,
                [1, 0, 1, 1, 0, 1, 1] => 3,
                [0, 1, 1, 1, 0, 1, 0] => 4,
                [1, 1, 0, 1, 0, 1, 1] => 5,
                [1, 1, 0, 1, 1, 1, 1] => 6,
                [1, 0, 1, 0, 0, 1, 0] => 7,
                [1, 1, 1, 1, 1, 1, 1] => 8,
                [1, 1, 1, 1, 0, 1, 1] => 9,
                _ => panic!("Unable to decode {}", s),
            };

            val_as_string.push(char::from_digit(digit, 10).unwrap());
        }

        i32::from_str(&val_as_string).unwrap()
    }
}

trait AssertUnique<T> {
    fn assert_unique(self) -> T;
}

impl<T, S: Iterator<Item = T>> AssertUnique<T> for S {
    fn assert_unique(self) -> T {
        let mut vec = self.collect::<Vec<_>>();
        assert_eq!(1, vec.len());
        vec.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::read_puzzle_input;

    #[test]
    fn solve_part_1() {
        let input = read_puzzle_input("day8-puzzle-input.txt");
        let mut input = parse_input(&input);
        let count = input
            .iter()
            .map(|l| {
                l.1.iter()
                    .filter(|d| d.len() == 2 || d.len() == 4 || d.len() == 3 || d.len() == 7)
                    .count() as i32
            })
            .sum();
        assert_eq!(383, count);
    }

    #[test]
    fn solve_part_2() {
        let input = read_puzzle_input("day8-puzzle-input.txt");
        let mut input = parse_input(&input);
        let mut sum = 0;

        for line in input.iter() {
            let decoder = build_decoder(&line.0);
            sum += decoder.decode_slice(&line.1);
        }

        assert_eq!(998900, sum);
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        );

        assert_eq!(input.len(), 1);

        let (unique, output) = input.first().unwrap();
        assert_eq!(unique.len(), 10);
        assert_eq!(output.len(), 4);
    }

    /// Tests that a valid decoder can be built from the actual encodings,
    /// assuming no wire swaps.
    #[test]
    fn test_build_decoder_identity() {
        // 0 1 2 3 4 5 6 7 8 9
        let input =
            parse_input("abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg | cf cf cf cf");
        let decoder = build_decoder(&input.first().unwrap().0);
    }

    #[test]
    fn test_deduce_wire_mapping() {
        let input = parse_input(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        let decoder = build_decoder(&input.first().unwrap().0);
        assert_eq!(5, decoder.decode("cdfeb"));
        assert_eq!(3, decoder.decode("fcadb"));
    }
}
