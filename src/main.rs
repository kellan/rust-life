use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::fmt;
use std::io;
use std::time::Duration;

// most logic lives on board
#[derive(Debug)]
struct Board {
    height: u8,
    width: u8,
    map: Vec<Vec<Cell>>,
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Dead(u8),
    Live(u8),
}

// I probably don't need this and should just use a tuple
#[derive(PartialEq, Eq, Hash, Debug)]
struct Point(i8, i8);

impl Point {
    fn from_u8(x: u8, y: u8) -> Point {
        Point(x as i8, y as i8)
    }
}

impl Board {
    fn new(height: u8, width: u8) -> Board {
        let board = Board {
            height: height,
            width: width,
            map: vec![vec![Cell::Dead(0); usize::from(width)]; usize::from(height)],
        };

        board
    }

    fn animate(&mut self) -> Result<()> {
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            SetForegroundColor(Color::Magenta),
            Hide
        )?;

        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(_) => break,
                    _ => {}
                }
            } else {
                execute!(
                    io::stdout(),
                    Clear(ClearType::All),
                    MoveTo(0, 0),
                    Print(&self),
                    Print("Press enter to exit...")
                )?;
                self.step();
            }
        }
        execute!(io::stdout(), ResetColor, Show, LeaveAlternateScreen)?;
        Ok(())
    }

    // flip a cell to alive
    fn revive(&mut self, hw: Point) {
        self.map[hw.0 as usize][hw.1 as usize] = Cell::Live(0);
    }

    fn at(&self, hw: &Point) -> Cell {
        self.map[hw.0 as usize][hw.1 as usize]
    }

    fn set(&mut self, hw: &Point, cell: Cell) {
        self.map[hw.0 as usize][hw.1 as usize] = cell;
    }

    fn step(&mut self) {
        self.update_neighbor_counts();
        self.update_cell_status();
    }

    fn update_neighbor_counts(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let p = Point::from_u8(i, j);
                let cell = self.at(&p);
                match cell {
                    Cell::Dead(_) => self.set(&p, Cell::Dead(self.living_neighbors(&p))),
                    Cell::Live(_) => self.set(&p, Cell::Live(self.living_neighbors(&p))),
                }
            }
        }
    }

    fn update_cell_status(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let p = Point::from_u8(i, j);
                let cell = self.at(&p);
                match cell {
                    Cell::Dead(3) => self.set(&p, Cell::Live(0)),
                    Cell::Dead(_) => self.set(&p, Cell::Dead(0)),
                    Cell::Live(0) | Cell::Live(1) => self.set(&p, Cell::Dead(0)),
                    Cell::Live(2) | Cell::Live(3) => self.set(&p, Cell::Live(0)),
                    Cell::Live(_) => {
                        // greater than 3
                        self.set(&p, Cell::Dead(0))
                    }
                }
            }
        }
    }

    fn living_neighbors(&self, hw: &Point) -> u8 {
        let mut living_count = 0;
        for p in self.neighbor_points(&hw) {
            if let Cell::Live(_) = self.at(&p) {
                living_count += 1;
            }
        }

        living_count
    }

    fn neighbor_points(&self, hw: &Point) -> Vec<Point> {
        let mut neighbors = Vec::<Point>::new();
        let height = self.height as i8;
        let width = self.width as i8;

        const DIRECTIONS: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let symbol = match self.at(&&Point::from_u8(i, j)) {
                    Cell::Dead(_) => '◻',
                    Cell::Live(_) => '◼',
                };

                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    let mut board = Board::new(3, 3);
    board.revive(Point(0, 0));
    board.revive(Point(1, 1));
    board.revive(Point(2, 0));
    board.revive(Point(2, 2));
    board.animate();

    // board.step();
    // board.step();

    //   print!("{}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test() {
        let empty_board = Board::new(1, 1);
        let cells = empty_board.neighbor_points(&Point(0, 0));
        assert_eq!(cells.len(), 0);

        let board = Board::new(3, 3);
        let cells = board.neighbor_points(&Point(0, 0));
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
        assert!(matches!(board.map[0][1], Cell::Dead(_)));

        assert_eq!(board.living_neighbors(&Point(0, 0)), 0);
        assert_eq!(board.living_neighbors(&Point(0, 1)), 1);
        board.revive(Point(0, 0));
        assert_eq!(board.living_neighbors(&Point(0, 1)), 1);
        board.revive(Point(0, 1));
        assert_eq!(board.living_neighbors(&Point(0, 1)), 1);
        board.revive(Point(1, 1));
        assert_eq!(board.living_neighbors(&Point(0, 1)), 2);
    }

    #[test]
    fn step_tests() {
        let mut board = Board::new(3, 3);
        board.revive(Point(0, 0));
        board.revive(Point(1, 1));
        board.revive(Point(2, 0));
        board.revive(Point(2, 2));

        board.step();

        assert!(matches!(board.map[1][0], Cell::Live(_)));
        assert!(matches!(board.map[1][1], Cell::Live(_)));
        assert!(matches!(board.map[2][1], Cell::Live(_)));

        board.step();

        assert!(matches!(board.map[1][0], Cell::Live(_)));
        assert!(matches!(board.map[1][1], Cell::Live(_)));
        assert!(matches!(board.map[2][1], Cell::Live(_)));
        assert!(matches!(board.map[2][0], Cell::Live(_)));
    }
}
