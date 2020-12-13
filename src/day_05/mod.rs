struct Seat {
    row: u32,
    column: u32,
    id: u32,
}

fn get_seat(input: &str) -> Seat {
    let mut row_min: u32 = 0;
    let mut row_max: u32 = 127;
    for c in input.get(0..7).unwrap().chars() {
        println!("row_min {}, row_max {}, c {}", row_min, row_max, c);
        match c {
            'F' => row_max = row_min + ((row_max - row_min) / 2),
            'B' => row_min = row_min + ((row_max - row_min + 1) / 2),
            _ => panic!("Needs to get 7 F or B chars first!"),
        }
    }
    println!("row_min {}, row_max {}", row_min, row_max);
    let mut col_min: u32 = 0;
    let mut col_max: u32 = 7;
    for c in input.get(7..10).unwrap().chars() {
        println!("col_min {}, col_max {}, c {}", col_min, col_max, c);
        match c {
            'L' => col_max = col_min + ((col_max - col_min) / 2),
            'R' => col_min = col_min + ((col_max - col_min + 1) / 2),
            _ => panic!("Needs to get 3 R or L chars in the end!"),
        }
    }
    println!("col_min {}, col_max {}", col_min, col_max);
    Seat {
        row: row_min,
        column: col_min,
        id: row_min * 8 + col_min,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn check_example_00() {
        let input = "FBFBBFFRLR";
        let result = get_seat(input);
        assert_eq!(44, result.row, "row should be 44");
        assert_eq!(5, result.column, "column should be 5");
        assert_eq!(357, result.id, "row should be 357");
    }

    #[test]
    fn check_example_01() {
        let input = "BFFFBBFRRR";
        let result = get_seat(input);
        assert_eq!(70, result.row, "row should be 70");
        assert_eq!(7, result.column, "column should be 7");
        assert_eq!(567, result.id, "row should be 567");
    }

    #[test]
    fn check_example_02() {
        let input = "FFFBBBFRRR";
        let result = get_seat(input);
        assert_eq!(14, result.row, "row should be 14");
        assert_eq!(7, result.column, "column should be 7");
        assert_eq!(119, result.id, "id should be 199");
    }

    #[test]
    fn check_example_03() {
        let input = "BBFFBBFRLL";
        let result = get_seat(input);
        assert_eq!(102, result.row, "row should be 102");
        assert_eq!(4, result.column, "column should be 4");
        assert_eq!(820, result.id, "id should be 820");
    }
}
