enum Tile {
    Empty,
    Tree,
    Tent,
}

struct Side {
    max_tents: usize,
    cur_tents: usize,
}

struct Level {
    rows: Vec<Side>,
    cols: Vec<Side>,
    tiles: Vec<Vec<Tile>>,
}

struct Symmetry {
    x_flip: bool,
    y_flip: bool,
    cw_90: bool,
    ccw_90: bool,
    rot_180: bool,
    uyx_flip: bool,
    iyz_flip: bool,
}
type Point = (usize, usize);

#[derive(Clone, PartialEq)]
struct Grid<T: Clone + Copy + PartialEq> {
    n: usize,
    grid: Vec<Vec<T>>,
}
impl<T: Clone + Copy + PartialEq> Grid<T> {
    // fn directional()
    // fn digotional()
    fn square(&mut self, point: Point, buf: &mut Vec<Point>, n: usize, wrapping: bool) {
        let (x, y) = point;
        let [left, up] = if wrapping {
            [x, y].map(|i| (self.n - n % self.n + i) % self.n)
        } else {
            [x, y].map(|i| i.saturating_sub(n))
        };
        let [right, down] = if wrapping {
            [x, y].map(|i| (i + n) % self.n)
        } else {
            [x, y].map(|i| usize::min(self.n - 1, i + n))
        };

        macro_rules! updown {
            ($i:expr) => {
                if up >= down {
                    (up..self.n).for_each(|j| buf.push(($i, j)));
                    (0..=down).for_each(|j| buf.push(($i, j)));
                } else {
                    (up..=down).for_each(|j| buf.push(($i, j)));
                }
            };
        }
        if left >= right {
            for i in left..self.n {
                updown!(i);
            }
            for i in 0..=right {
                updown!(i);
            }
        } else {
            for i in left..=right {
                updown!(i);
            }
        }
    }
}