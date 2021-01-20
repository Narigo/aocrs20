use std::fmt;

enum Cell {
    Occupied,
    Empty,
    Floor,
}

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
    fn check_input_day_11() {
        let file = read_file("./src/day_11/input.txt");
        let grid = input_to_grid(&file);
        assert_eq!(99, grid.height);
        assert_eq!(90, grid.width);
    }
}
