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

fn input_to_grid(input: &str) -> Grid {
    let cells = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let width = lines.get(0).unwrap_or(&"").len();
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
