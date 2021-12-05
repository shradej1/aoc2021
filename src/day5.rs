//! --- Day 5: Hydrothermal Venture ---
//!
//! You come across a field of hydrothermal vents on the ocean floor! These
//! vents constantly produce large, opaque clouds, so it would be best to avoid
//! them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of
//! nearby lines of vents (your puzzle input) for you to review. For example:
//!
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//!
//! Each line of vents is given as a line segment in the format x1,y1 -> x2,y2
//! where x1,y1 are the coordinates of one end the line segment and x2,y2 are
//! the coordinates of the other end. These line segments include the points at
//! both ends. In other words:
//!
//!     An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//!     An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//!
//! For now, only consider horizontal and vertical lines: lines where either x1
//! = x2 or y1 = y2.
//!
//! So, the horizontal and vertical lines from the above list would produce the
//! following diagram:
//!
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//!
//! In this diagram, the top left corner is 0,0 and the bottom right corner is
//! 9,9. Each position is shown as the number of lines which cover that point or
//! . if no line covers that point. The top-left pair of 1s, for example, comes
//! from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9
//! -> 5,9 and 0,9 -> 2,9.
//!
//! To avoid the most dangerous areas, you need to determine the number of
//! points where at least two lines overlap. In the above example, this is
//! anywhere in the diagram with a 2 or larger - a total of 5 points.
//!
//! Consider only horizontal and vertical lines. At how many points do at least
//! two lines overlap?
//!
//! --- Part Two ---
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give
//! you the full picture; you need to also consider diagonal lines.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in
//! your list will only ever be horizontal, vertical, or a diagonal line at
//! exactly 45 degrees. In other words:
//!
//!     An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//!     An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//!
//! Considering all lines from the above example would now produce the following
//! diagram:
//!
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//!
//! You still need to determine the number of points where at least two lines
//! overlap. In the above example, this is still anywhere in the diagram with a
//! 2 or larger - now a total of 12 points.
//!
//! Consider all of the lines. At how many points do at least two lines overlap?

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Defines a line by its inclusive endpoints.
#[derive(Copy, Clone, Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn is_vertial(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
}

struct HorizontalIterator {
    line: Line,
    next: usize,
    size: usize,
}

impl HorizontalIterator {
    fn new(line: Line) -> Self {
        assert_eq!(line.y1, line.y2, "Line is not horizontal");

        // switch x1 and x2 if necessary so that we always iterate starting at the
        // smallest.
        let (max, min) = if line.x1 > line.x2 {
            (line.x1, line.x2)
        } else {
            (line.x2, line.x1)
        };

        Self {
            line: Line {
                x1: min,
                y1: line.y1,
                x2: max,
                y2: line.y2,
            },
            next: 0,
            size: max - min + 1,
        }
    }
}

impl Iterator for HorizontalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.size {
            let x = self.line.x1 + self.next;
            let y = self.line.y1;
            self.next += 1;
            Some((x, y))
        } else {
            None
        }
    }
}

impl From<Line> for HorizontalIterator {
    fn from(line: Line) -> Self {
        HorizontalIterator::new(line)
    }
}

struct VerticalIterator {
    line: Line,
    next: usize,
    size: usize,
}

impl VerticalIterator {
    fn new(line: Line) -> Self {
        assert_eq!(line.x1, line.x2, "Line is not vertical");

        // switch y1 and y2 if necessary so that we always iterate starting at the
        // smallest.
        let (max, min) = if line.y1 > line.y2 {
            (line.y1, line.y2)
        } else {
            (line.y2, line.y1)
        };

        Self {
            line: Line {
                x1: line.x1,
                y1: min,
                x2: line.x2,
                y2: max,
            },
            next: 0,
            size: max - min + 1,
        }
    }
}

impl Iterator for VerticalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.size {
            let x = self.line.x1;
            let y = self.line.y1 + self.next;
            self.next += 1;
            Some((x, y))
        } else {
            None
        }
    }
}

impl From<Line> for VerticalIterator {
    fn from(line: Line) -> Self {
        VerticalIterator::new(line)
    }
}

struct DiagonalIterator {
    line: Line,
    next: usize,
    size: usize,
}

impl DiagonalIterator {
    fn new(line: Line) -> Self {
        assert_eq!(
            (line.y2 as i32 - line.y1 as i32).abs(),
            (line.x2 as i32 - line.x1 as i32).abs(),
            "Line is not 45-degree diagonal"
        );

        Self {
            line,
            next: 0,
            size: usize::try_from((line.x2 as i32 - line.x1 as i32).abs() + 1).unwrap(),
        }
    }
}

impl Iterator for DiagonalIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.size {
            let n = self.next as i32;
            let x = self.line.x1 as i32 + if self.line.x1 < self.line.x2 { n } else { -n };
            let y = self.line.y1 as i32 + if self.line.y1 < self.line.y2 { n } else { -n };
            self.next += 1;
            Some((usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
        } else {
            None
        }
    }
}

impl From<Line> for DiagonalIterator {
    fn from(line: Line) -> Self {
        DiagonalIterator::new(line)
    }
}

/// Parses the input
fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut toks = l.split(',');
            let x1 = usize::from_str(toks.next().unwrap().trim()).unwrap();
            let mut inner = toks.next().unwrap().split("->");
            let y1 = usize::from_str(inner.next().unwrap().trim()).unwrap();
            let x2 = usize::from_str(inner.next().unwrap().trim()).unwrap();
            let y2 = usize::from_str(toks.next().unwrap().trim()).unwrap();

            Line { x1, x2, y1, y2 }
        })
        .collect()
}

struct Cover {
    /// Counts the number of intersections at each point.
    counts: Vec<usize>,
    width: usize,
    height: usize,
    stride: usize,
}

impl Cover {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Cover {
            counts: vec![0; size],
            width,
            height,
            stride: width,
        }
    }

    /// Gets the number of lines that cover the given (x, y) point.
    fn count(&self, x: usize, y: usize) -> usize {
        self.counts[x * self.stride + y]
    }

    /// Covers the point (x, y).
    fn cover(&mut self, x: usize, y: usize) {
        self.counts[x * self.stride + y] += 1;
    }

    fn add_line(&mut self, line: Line) {
        let iter: Box<dyn Iterator<Item = (usize, usize)>> = if line.is_vertial() {
            Box::new(VerticalIterator::from(line))
        } else if line.is_horizontal() {
            Box::new(HorizontalIterator::from(line))
        } else {
            Box::new(DiagonalIterator::from(line))
        };

        iter.for_each(|p| self.cover(p.0, p.1));
    }
}

fn build_cover(input: impl Iterator<Item = Line> + Clone) -> Cover {
    let width = input.clone().map(|l| l.x1.max(l.x2)).max().unwrap() + 1;
    let height = input.clone().map(|l| l.y1.max(l.y2)).max().unwrap() + 1;
    let mut cover = Cover::new(width, height);
    input.for_each(|l| cover.add_line(l));
    cover
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_puzzle_input;

    #[test]
    fn solve_part_1() {
        let input = parse_input(&read_puzzle_input("day5-puzzle-input.txt"));
        let cover = build_cover(
            input
                .into_iter()
                .filter(|l| l.is_vertial() || l.is_horizontal()),
        );
        let points_covered_by_two_or_more = cover.counts.iter().filter(|c| **c >= 2).count();
        assert_eq!(points_covered_by_two_or_more, 6856);
    }

    #[test]
    fn solve_part_2() {
        let input = parse_input(&read_puzzle_input("day5-puzzle-input.txt"));
        let cover = build_cover(input.into_iter());
        let points_covered_by_two_or_more = cover.counts.iter().filter(|c| **c >= 2).count();
        assert_eq!(points_covered_by_two_or_more, 20666);
    }

    #[test]
    fn test_cover() {
        let input = parse_input(
            r"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2",
        );

        let cover = build_cover(
            input
                .into_iter()
                .filter(|l| l.is_vertial() || l.is_horizontal()),
        );
        assert_eq!(10, cover.width);
        assert_eq!(10, cover.height);
        assert_eq!(10, cover.stride);

        assert_eq!(cover.count(0, 0), 0);
        assert_eq!(cover.count(1, 0), 0);
        assert_eq!(cover.count(2, 0), 0);
        assert_eq!(cover.count(3, 0), 0);
        assert_eq!(cover.count(4, 0), 0);
        assert_eq!(cover.count(5, 0), 0);
        assert_eq!(cover.count(6, 0), 0);
        assert_eq!(cover.count(7, 0), 1);
        assert_eq!(cover.count(8, 0), 0);
        assert_eq!(cover.count(9, 0), 0);
    }

    #[test]
    fn test_horizontal_iterator() {
        let horiz = Line {
            x1: 0,
            y1: 0,
            x2: 7,
            y2: 0,
        };

        assert!(horiz.is_horizontal());
        let mut iter = HorizontalIterator::from(horiz);
        assert_eq!((0, 0), iter.next().unwrap());
        assert_eq!((1, 0), iter.next().unwrap());
        assert_eq!((2, 0), iter.next().unwrap());
        assert_eq!((3, 0), iter.next().unwrap());
        assert_eq!((4, 0), iter.next().unwrap());
        assert_eq!((5, 0), iter.next().unwrap());
        assert_eq!((6, 0), iter.next().unwrap());
        assert_eq!((7, 0), iter.next().unwrap());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_diagonal_iterator() {
        let diag = Line {
            x1: 0,
            y1: 0,
            x2: 5,
            y2: 5,
        };

        let mut iter = DiagonalIterator::from(diag);
        assert_eq!((0, 0), iter.next().unwrap());
        assert_eq!((1, 1), iter.next().unwrap());
        assert_eq!((2, 2), iter.next().unwrap());
        assert_eq!((3, 3), iter.next().unwrap());
        assert_eq!((4, 4), iter.next().unwrap());
        assert_eq!((5, 5), iter.next().unwrap());
        assert!(iter.next().is_none());
    }
}
