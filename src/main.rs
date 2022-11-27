use std::collections::HashMap;

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Board {
    height: i8,
    width: i8,
    map: HashMap<Point, bool>,
}

#[derive(Debug)]
struct Cell {
    alive: bool,
    neighbors: i8,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i8, i8);

impl Board {
    fn new(height: i8, width: i8) -> Board {
        let board = Board {
            height: height,
            width: width,
            map: HashMap::new(),
        };

        board
    }

    fn cell(&self, hw: &Point) -> bool {
        *self.map.get(&hw).unwrap_or(&false)
    }

    fn neighbors(&self, hw: Point) -> Vec<Point> {
        let mut neighbors = Vec::<Point>::new();
        for delta in DIRECTIONS {
            let dh = hw.0 - delta.0;
            let dw = hw.1 - delta.1;

            if (dh >= 0 && dh < self.height) && (dw >= 0 && dw < self.width) {
                neighbors.push(Point(dh, dw));
            }
        }

        neighbors
    }
}

fn main() {
    println!("Hello, world!");
    let board = Board::new(10, 10);
    dbg!(&board);
    let cells = board.neighbors(Point(1, 1));
    dbg!(&cells);
    dbg!(board.cell(&cells[1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test() {
        let empty_board = Board::new(1, 1);
        let cells = empty_board.neighbors(Point(0, 0));
        assert_eq!(cells.len(), 0);

        let board = Board::new(3, 3);
        let cells = board.neighbors(Point(0, 0));
        assert_eq!(cells.len(), 3);
        assert!(cells.contains(&Point(0, 1)));
        assert!(!cells.contains(&Point(1, 2)));
    }
}
