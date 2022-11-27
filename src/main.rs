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

#[derive(Clone, Debug)]
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

    // fn insert_cell(&mut self, hw: Point) {
    //     self.map.insert(hw, Cell::new());
    // }

    // fn living_neighbors(&self, hw: Point) -> Vec<&Cell> {
    //     let points = self.neighbor_points(hw);
    //     let mut cells = Vec::<&Cell>::new();
    //     for p in points {
    //         if let Some(cell) = self.map.get(&p) {
    //             cells.push(cell);
    //         }
    //     }

    //     cells
    // }

    // // fn neighbor_cells(&self, hw: Point) -> Vec<&Cell> {
    // //     let points = self.neighbor_points(hw);
    // //     let mut cells = Vec::<&Cell>::new();
    // //     for p in points {
    // //         if self.map.contains_key(&p) {
    // //             cells.push(self.map.get(&p).unwrap());
    // //         }
    // //     }

    // //     cells
    // // }

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

    // #[test]
    // fn cell_tests() {
    //     let mut board = Board::new(3, 3);
    //     let living = board.living_neighbors(Point(0, 0));
    //     assert_eq!(living.len(), 0);
    //     board.insert_cell(Point(0, 1));
    //     let cell = board.map.get(&Point(0, 1));
    //     assert!(cell.is_some());
    //     let living = board.living_neighbors(Point(0, 0));
    //     assert_eq!(living.len(), 1);
    // }
}
