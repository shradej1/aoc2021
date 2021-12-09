use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::str::FromStr;

struct HeightMap {
    rows: usize,
    cols: usize,
    height: Vec<u32>,
}

impl HeightMap {
    /// Computes the sum of the low point risk levels.
    fn compute_sum_of_low_point_risk_levels(&self) -> u32 {
        let mut sum = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.is_low_point(row, col) {
                    sum += self.height_at(row, col).unwrap() + 1;
                }
            }
        }
        sum
    }

    /// The size of a basin is the number of locations within the basin,
    /// including the low point.
    fn order_basin_sizes(&self) -> Vec<usize> {
        // map each low point to the size of the basin that it sits in
        let mut basins = (0..self.rows)
            .flat_map(|r| {
                (0..self.cols)
                    .map(move |c| (r, c))
                    .filter(|(r, c)| self.is_low_point(*r, *c))
                    .map(|(r, c)| self.find_basin_size(r, c))
            })
            .collect::<Vec<_>>();
        basins.sort();
        basins
    }

    /// Returns the points that make up the basin that row/col resides
    fn find_basin_size(&self, row: usize, col: usize) -> usize {
        fn find_basin_inner(
            height_map: &HeightMap,
            new_points: Vec<(usize, usize)>,
            agg: &mut BTreeSet<(usize, usize)>,
        ) -> usize {
            let mut new_new_points = Vec::new();
            for (row, col) in new_points {
                agg.insert((row, col));
                let height = height_map.height_at(row, col).unwrap();
                let adjacents = height_map.wrapping_adjacent_coords(row, col);
                adjacents
                    .iter()
                    .filter(|(row, col)| {
                        height_map
                            .height_at(*row, *col)
                            .map(|h| h > height && h != 9)
                            .unwrap_or(false)
                    })
                    .for_each(|a| {
                        new_new_points.push(*a);
                    })
            }

            if new_new_points.is_empty() {
                agg.len()
            } else {
                find_basin_inner(&height_map, new_new_points, agg)
            }
        }

        let mut agg = BTreeSet::new();
        find_basin_inner(self, vec![(row, col)], &mut agg);
        agg.len()
    }

    fn height_at(&self, row: usize, col: usize) -> Option<u32> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.height[row * self.cols + col])
        }
    }

    fn adjacents(&self, row: usize, col: usize) -> [Option<u32>; 4] {
        [
            self.height_at(row, col + 1),
            self.height_at(row, usize::wrapping_sub(col, 1)),
            self.height_at(row + 1, col),
            self.height_at(usize::wrapping_sub(row, 1), col),
        ]
    }

    fn wrapping_adjacent_coords(&self, row: usize, col: usize) -> [(usize, usize); 4] {
        [
            (row, col + 1),
            (row, usize::wrapping_sub(col, 1)),
            (row + 1, col),
            (usize::wrapping_sub(row, 1), col),
        ]
    }

    fn is_low_point(&self, row: usize, col: usize) -> bool {
        let height = self.height_at(row, col).unwrap();
        self.adjacents(row, col)
            .iter()
            .filter(|o| o.is_some())
            .all(|a| a.unwrap() > height)
    }
}

fn parse_input(input: &str) -> HeightMap {
    let rows = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .count();
    let cols = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .next()
        .unwrap()
        .len();
    let height = input
        .trim()
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| char::to_digit(c, 10).expect(&format!("Couldn't parse '{}'", c)))
        .collect();
    HeightMap { rows, cols, height }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::read_puzzle_input;

    #[test]
    fn solve_part_1() {
        let input = read_puzzle_input("day9-puzzle-input.txt");
        let mut input = parse_input(&input);
        assert_eq!(475, input.compute_sum_of_low_point_risk_levels());
    }

    #[test]
    fn solve_part_2() {
        let input = read_puzzle_input("day9-puzzle-input.txt");
        let mut input = parse_input(&input);
        assert_eq!(
            1092012,
            input
                .order_basin_sizes()
                .iter()
                .rev()
                .take(3)
                .cloned()
                .map(|u| u as i32)
                .product()
        )
    }

    #[test]
    fn test_example_1() {
        let input = r"2199943210
            3987894921
            9856789892
            8767896789
            9899965678";

        let input = parse_input(input);
        assert_eq!(input.rows, 5);
        assert_eq!(input.cols, 10);
        let sizes = input.order_basin_sizes();
        assert_eq!(&[3, 9, 9, 14], sizes.as_slice());
    }
}
