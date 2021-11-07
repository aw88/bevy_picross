pub struct Solution {
    pub size: (usize, usize),
    pub tiles: Vec<Vec<u8>>,
}

impl Solution {
    pub fn new(size: (usize, usize), tiles: Vec<Vec<u8>>) -> Self {
        Self {
            size,
            tiles,
        }
    }

    pub fn lookup(&self, tile_coord: (usize, usize)) -> bool {
        let (x, y) = tile_coord;

        *self.tiles.get(y)
            .and_then(|r| r.get(x))
            .unwrap_or(&0) > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;

    #[test]
    fn lookup_tile_out_of_bounds() {
        let solution = Solution::new(
            (4, 4),
            vec![
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
            ],
        );

        assert_eq!(solution.lookup((5, 1)), false);
        assert_eq!(solution.lookup((3, 4)), false);
    }

    #[test]
    fn lookup_tile_in_bounds() {
        let solution = Solution::new(
            (4, 4),
            vec![
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
            ],
        );

        assert_eq!(solution.lookup((0, 0)), false);
        assert_eq!(solution.lookup((1, 2)), true);
        assert_eq!(solution.lookup((3, 3)), false);
    }
}