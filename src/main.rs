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
    height: u8,
    width: u8,
    map: Vec<Vec<Cell>>,
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Dead,
    Live(i8),
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i8, i8);

impl Board {
    fn new(height: u8, width: u8) -> Board {
        let board = Board {
            height: height,
            width: width,
            map: vec![vec![Cell::Dead; usize::from(width)]; usize::from(height)],
        };

        board
    }

    fn revive(&mut self, hw: Point) {
        self.map[hw.0 as usize][hw.1 as usize] = Cell::Live(0);
    }

    fn at(&self, hw: &Point) -> Cell {
        self.map[hw.0 as usize][hw.1 as usize]
    }

    fn living_neighbors(&self, hw: Point) -> u8 {
        let mut living_count = 0;
        for p in self.neighbor_points(hw) {
            if let Cell::Live(_) = self.at(&p) {
                living_count += 1;
            }
        }

        living_count
    }

    fn neighbor_points(&self, hw: Point) -> Vec<Point> {
        let mut neighbors = Vec::<Point>::new();
        let height = self.height as i8;
        let width = self.width as i8;

        for delta in DIRECTIONS {
            let dh = hw.0 - delta.0;
            let dw = hw.1 - delta.1;

            if (dh >= 0 && dh < height) && (dw >= 0 && dw < width) {
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
    let cells = board.neighbor_points(Point(1, 1));
    dbg!(&cells);
    //    dbg!(board.cell(&cells[1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test() {
        let empty_board = Board::new(1, 1);
        let cells = empty_board.neighbor_points(Point(0, 0));
        assert_eq!(cells.len(), 0);

        let board = Board::new(3, 3);
        let cells = board.neighbor_points(Point(0, 0));
        assert_eq!(cells.len(), 3);
        assert!(cells.contains(&Point(0, 1)));
        assert!(!cells.contains(&Point(1, 2)));
    }

    #[test]
    fn cell_tests() {
        let mut board = Board::new(3, 3);
        board.revive(Point(0, 0));
        assert!(matches!(board.map[0][0], Cell::Live(_)));
        assert!(!matches!(board.map[0][1], Cell::Live(_)));
        assert!(matches!(board.map[0][1], Cell::Dead));

        assert_eq!(board.living_neighbors(Point(0, 0)), 0);
        assert_eq!(board.living_neighbors(Point(0, 1)), 1);
        board.revive(Point(0, 0));
        assert_eq!(board.living_neighbors(Point(0, 1)), 1);
        board.revive(Point(0, 1));
        assert_eq!(board.living_neighbors(Point(0, 1)), 1);
        board.revive(Point(1, 1));
        assert_eq!(board.living_neighbors(Point(0, 1)), 2);
    }
}
