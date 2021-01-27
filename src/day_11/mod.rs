use std::fmt;

#[derive(Debug, PartialEq)]
enum Cell {
    Occupied,
    Empty,
    Floor,
}

#[derive(Debug, PartialEq)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Occupied => write!(f, "#"),
            Cell::Empty => write!(f, "L"),
            Cell::Floor => write!(f, "."),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

trait GridRules {
    fn get_occupied_adjacent_cells(&self, x: usize, y: usize) -> usize;
    fn get_next_state_of_cell(&self, x: usize, y: usize) -> Cell;
    fn get_next_state(&self) -> Grid;
    fn get_last_state(&self) -> Grid;
    fn get_number_of_occupied_seats(&self) -> usize;
}

impl GridRules for Grid {
    fn get_next_state_of_cell(&self, x: usize, y: usize) -> Cell {
        self.cells
            .get(y)
            .and_then(|row| row.get(x))
            .map(|cell| match cell {
                Cell::Floor => Cell::Floor,
                Cell::Empty => {
                    if self.get_occupied_adjacent_cells(x, y) == 0 {
                        Cell::Occupied
                    } else {
                        Cell::Empty
                    }
                }
                Cell::Occupied => {
                    if self.get_occupied_adjacent_cells(x, y) >= 4 {
                        Cell::Empty
                    } else {
                        Cell::Occupied
                    }
                }
            })
            .unwrap_or(Cell::Floor)
    }

    fn get_occupied_adjacent_cells(&self, x: usize, y: usize) -> usize {
        let possible_cells = vec![
            y.checked_sub(1)
                .and_then(|y| self.cells.get(y))
                .and_then(|row| x.checked_sub(1).and_then(|x| row.get(x))),
            y.checked_sub(1)
                .and_then(|y| self.cells.get(y))
                .and_then(|row| row.get(x)),
            y.checked_sub(1)
                .and_then(|y| self.cells.get(y))
                .and_then(|row| row.get(x + 1)),
            self.cells
                .get(y)
                .and_then(|row| x.checked_sub(1).and_then(|x| row.get(x))),
            self.cells.get(y).and_then(|row| row.get(x + 1)),
            self.cells
                .get(y + 1)
                .and_then(|row| x.checked_sub(1).and_then(|x| row.get(x))),
            self.cells.get(y + 1).and_then(|row| row.get(x)),
            self.cells.get(y + 1).and_then(|row| row.get(x + 1)),
        ];
        possible_cells.iter().fold(0, |acc, c| {
            acc + match c {
                Some(Cell::Occupied) => 1,
                _ => 0,
            }
        })
    }

    fn get_next_state(&self) -> Grid {
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for y in 0..self.width {
            let mut cells: Vec<Cell> = Vec::new();
            for x in 0..self.height {
                cells.push(self.get_next_state_of_cell(x, y));
            }
            grid.push(cells);
        }
        Grid {
            cells: grid,
            height: self.height,
            width: self.width,
        }
    }

    fn get_last_state(&self) -> Grid {
        let next_grid = self.get_next_state();
        if next_grid == *self {
            next_grid
        } else {
            next_grid.get_last_state()
        }
    }

    fn get_number_of_occupied_seats(&self) -> usize {
        0
    }
}

fn input_to_grid(input: &str) -> Grid {
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines.get(0).unwrap_or(&"").len();
    let mut cells: Vec<Vec<Cell>> = Vec::new();

    for line in lines.iter() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                'L' => row.push(Cell::Empty),
                '#' => row.push(Cell::Occupied),
                _ => row.push(Cell::Floor),
            }
        }
        cells.push(row);
    }

    Grid {
        cells: cells,
        height: height,
        width: width,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example1_day_11() {
        let file = read_file("./src/day_11/example.txt");
        let grid = input_to_grid(&file);
        println!("{}", grid);
        assert_eq!(10, grid.height);
        assert_eq!(10, grid.width);
    }

    #[test]
    fn check_adjacent_cells() {
        let file = read_file("./src/day_11/adjacent_cells.txt");
        let grid = input_to_grid(&file);
        assert_eq!(4, grid.get_occupied_adjacent_cells(1, 1));
        assert_eq!(2, grid.get_occupied_adjacent_cells(0, 0));
        assert_eq!(4, grid.get_occupied_adjacent_cells(1, 0));
        assert_eq!(2, grid.get_occupied_adjacent_cells(3, 2));
    }

    #[test]
    fn check_next_state_of_cell() {
        let file = read_file("./src/day_11/adjacent_cells.txt");
        let grid = input_to_grid(&file);
        assert_eq!(Cell::Empty, grid.get_next_state_of_cell(1, 1));
        assert_eq!(Cell::Occupied, grid.get_next_state_of_cell(0, 1));
        assert_eq!(Cell::Floor, grid.get_next_state_of_cell(1, 0));
        assert_eq!(Cell::Empty, grid.get_next_state_of_cell(0, 0));
        assert_eq!(Cell::Occupied, grid.get_next_state_of_cell(2, 3));
    }

    #[test]
    fn check_next_state_of_grid() {
        let file = read_file("./src/day_11/adjacent_cells.txt");
        let file2 = read_file("./src/day_11/adjacent_cells_2.txt");
        let grid = input_to_grid(&file);
        let grid2 = input_to_grid(&file2);
        let next_grid = grid.get_next_state();
        assert_eq!(grid2, next_grid);
    }

    #[test]
    fn check_next_states_of_grid_for_example() {
        let file1 = read_file("./src/day_11/example.txt");
        let file2 = read_file("./src/day_11/example_state_2.txt");
        let file3 = read_file("./src/day_11/example_state_3.txt");
        let file4 = read_file("./src/day_11/example_state_4.txt");
        let file5 = read_file("./src/day_11/example_state_5.txt");
        let file6 = read_file("./src/day_11/example_state_6.txt");
        let grid = input_to_grid(&file1);
        let grid2 = input_to_grid(&file2);
        let grid3 = input_to_grid(&file3);
        let grid4 = input_to_grid(&file4);
        let grid5 = input_to_grid(&file5);
        let grid6 = input_to_grid(&file6);
        let grid = grid.get_next_state();
        assert_eq!(grid2, grid);
        let grid = grid.get_next_state();
        assert_eq!(grid3, grid);
        let grid = grid.get_next_state();
        assert_eq!(grid4, grid);
        let grid = grid.get_next_state();
        assert_eq!(grid5, grid);
        let grid = grid.get_next_state();
        assert_eq!(grid6, grid);
    }

    #[test]
    fn check_last_state_of_grid_for_example() {
        let file = read_file("./src/day_11/example.txt");
        let last_file = read_file("./src/day_11/example_state_6.txt");
        let grid = input_to_grid(&file);
        let last_grid = input_to_grid(&last_file);
        let grid = grid.get_last_state();
        assert_eq!(last_grid, grid);
    }

    #[test]
    fn check_number_of_occupied_seats() {
        let file = read_file("./src/day_11/example_state_6.txt");
        let grid = input_to_grid(&file);
        let number_of_seats = grid.get_number_of_occupied_seats();
        assert_eq!(37, number_of_seats);
    }

    #[test]
    fn check_input_day_11() {
        let file = read_file("./src/day_11/input.txt");
        let grid = input_to_grid(&file);
        assert_eq!(99, grid.height);
        assert_eq!(90, grid.width);
    }
}
