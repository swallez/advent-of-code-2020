
const DATA: &str = include_str!("../../data/day_03.txt");

struct Terrain {
    width: usize,
    height: usize,
    // Matrix of ascii characters
    data: Vec<&'static [u8]>,
}

impl From<&'static str> for Terrain {
    fn from(s: &'static str) -> Self {
        let mut width_check: Option<usize> = None;
        let matrix: Vec<&[u8]> = s
            .lines()
            .inspect(|s| match width_check {
                None => width_check = Some(s.len()),
                Some(len) if len != s.len() => panic!("Inconsistent line length in input data"),
                _ => ()
            })
            .map(|s| s.as_bytes())
            .collect();

        Terrain {
            width: matrix[0].len(),
            height: matrix.len(),
            data: matrix,
        }
    }
}

impl Terrain {
    /// Are we on a tree? Returns None if we're past the bottom
    fn is_a_tree(&self, x: usize, y: usize) -> Option<bool> {
        if y >= (self.height) {
            None
        } else {
            Some(self.data[y][x % self.width] == b'#')
        }
    }

    fn run(&self, dx: usize, dy: usize) -> Trajectory {
        Trajectory {
            terrain: self,
            x: 0,
            y: 0,
            dx,
            dy,
        }
    }
}

/// An iterator of "is it a tree?" booleans over a trajectory.
struct Trajectory<'a> {
    terrain: &'a Terrain,
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

impl <'a> Iterator for Trajectory<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let r = self.terrain.is_a_tree(self.x, self.y);
        self.x += self.dx;
        self.y += self.dy;
        r
    }
}

fn count_trees(terrain: &Terrain, dx: usize, dy:usize) -> usize {
    terrain.run(dx, dy).filter(|tree| *tree).count()
}

fn mul_count(terrain: &Terrain, slopes: Vec<(usize, usize)>) -> usize {
    slopes.into_iter()
        .inspect(|(dx, dy)| print!("Trees hit with slope ({}, {}): ", dx, dy))
        .map(|(dx, dy)| count_trees(&terrain, dx, dy))
        .inspect(|x| println!("{}", x))
        .product()
}

fn main() {
    let terrain = Terrain::from(DATA);
    println!("Trees hit with slope (3, 1): {}", count_trees(&terrain, 3, 1));
    println!();

    let r = mul_count(&terrain, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    println!("Product: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../../data/day_03_test.txt");

    #[test]
    fn test_terrain() {
        let terrain = Terrain::from(TEST_DATA);
        let c = count_trees(&terrain, 3, 1);
        assert_eq!(c, 7);
    }

    #[test]
    fn test_mul_count() {
        let terrain = Terrain::from(TEST_DATA);
        let r = mul_count(&terrain, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
        assert_eq!(r, 336);
    }
}
