#[derive(Debug, PartialEq)]
enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Forward(i64),
    Left(i64),
    Right(i64),
}

fn input_to_commands(input: &String) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    use Command::*;
    for line in input.lines() {
        let (cmd, value) = line.split_at(1);
        let value = value.parse::<i64>().unwrap();
        let result = match cmd {
            "N" => North(value),
            "S" => South(value),
            "E" => East(value),
            "W" => West(value),
            "F" => Forward(value),
            "L" => Left(value),
            "R" => Right(value),
            _ => panic!("Unknown command"),
        };
        commands.push(result);
    }
    commands
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
struct Point {
    north_south: i64,
    east_west: i64,
}

#[derive(Clone)]
struct Ship {
    facing: Direction,
    current_distance: Point,
}

#[derive(Clone)]
struct Waypoint {
    north_south: i64,
    east_west: i64,
}

#[derive(Clone)]
struct ShipStar2 {
    waypoint: Waypoint,
    current_position: Point,
}

trait ShipFunctions
where
    Self: std::marker::Sized,
{
    fn new() -> Self;
    fn manhattan_distance(&self, commands: Vec<Command>) -> (Self, i64);
}

fn facing_to_angle(facing: Direction) -> i64 {
    match facing {
        Direction::East => 0,
        Direction::South => 90,
        Direction::West => 180,
        Direction::North => 270,
    }
}

fn angle_to_facing(angle: i64) -> Direction {
    let angle = (angle + 360) % 360;
    if angle == 0 {
        Direction::East
    } else if angle == 90 {
        Direction::South
    } else if angle == 180 {
        Direction::West
    } else if angle == 270 {
        Direction::North
    } else {
        panic!("Should have angles of 90 degrees, but got {}", angle);
    }
}

impl ShipFunctions for Ship {
    fn new() -> Ship {
        Ship {
            facing: Direction::East,
            current_distance: Point {
                north_south: 0,
                east_west: 0,
            },
        }
    }
    fn manhattan_distance(&self, commands: Vec<Command>) -> (Self, i64) {
        let mut current_ship = self.clone();
        for command in commands.iter() {
            current_ship = match command {
                Command::Left(angle) => Ship {
                    facing: angle_to_facing(facing_to_angle(current_ship.facing) - angle),
                    current_distance: current_ship.current_distance,
                },
                Command::Right(angle) => Ship {
                    facing: angle_to_facing(facing_to_angle(current_ship.facing) + angle),
                    current_distance: current_ship.current_distance,
                },
                Command::East(value) => Ship {
                    facing: current_ship.facing,
                    current_distance: Point {
                        north_south: current_ship.current_distance.north_south,
                        east_west: current_ship.current_distance.east_west + value,
                    },
                },
                Command::West(value) => Ship {
                    facing: current_ship.facing,
                    current_distance: Point {
                        north_south: current_ship.current_distance.north_south,
                        east_west: current_ship.current_distance.east_west - value,
                    },
                },
                Command::North(value) => Ship {
                    facing: current_ship.facing,
                    current_distance: Point {
                        north_south: current_ship.current_distance.north_south + value,
                        east_west: current_ship.current_distance.east_west,
                    },
                },
                Command::South(value) => Ship {
                    facing: current_ship.facing,
                    current_distance: Point {
                        north_south: current_ship.current_distance.north_south - value,
                        east_west: current_ship.current_distance.east_west,
                    },
                },
                Command::Forward(value) => {
                    let (ns_mult, ew_mult) = match current_ship.facing {
                        Direction::North => (1, 0),
                        Direction::South => (-1, 0),
                        Direction::East => (0, 1),
                        Direction::West => (0, -1),
                    };
                    Ship {
                        facing: current_ship.facing,
                        current_distance: Point {
                            north_south: current_ship.current_distance.north_south
                                + (ns_mult * value),
                            east_west: current_ship.current_distance.east_west + (ew_mult * value),
                        },
                    }
                }
            }
        }
        let distance = current_ship.current_distance.north_south.abs()
            + current_ship.current_distance.east_west.abs();
        (current_ship, distance)
    }
}

impl ShipFunctions for ShipStar2 {
    fn new() -> ShipStar2 {
        ShipStar2 {
            waypoint: Waypoint {
                north_south: 1,
                east_west: 10,
            },
            current_position: Point {
                north_south: 0,
                east_west: 0,
            },
        }
    }
    fn manhattan_distance(&self, commands: Vec<Command>) -> (ShipStar2, i64) {
        let mut ship = self.clone();
        for command in commands.iter() {
            let (next_waypoint, next_position) = match command {
                Command::North(value) => (
                    Waypoint {
                        north_south: ship.waypoint.north_south + value,
                        east_west: ship.waypoint.east_west,
                    },
                    ship.current_position,
                ),
                Command::South(value) => (
                    Waypoint {
                        north_south: ship.waypoint.north_south - value,
                        east_west: ship.waypoint.east_west,
                    },
                    ship.current_position,
                ),
                Command::East(value) => (
                    Waypoint {
                        north_south: ship.waypoint.north_south,
                        east_west: ship.waypoint.east_west + value,
                    },
                    ship.current_position,
                ),
                Command::West(value) => (
                    Waypoint {
                        north_south: ship.waypoint.north_south,
                        east_west: ship.waypoint.east_west - value,
                    },
                    ship.current_position,
                ),
                Command::Left(angle) => {
                    let x = (ship.waypoint.east_west * ((*angle as f64).to_radians().cos() as i64))
                        - (ship.waypoint.north_south * ((*angle as f64).to_radians().sin() as i64));
                    let y = (ship.waypoint.east_west * ((*angle as f64).to_radians().sin() as i64))
                        + (ship.waypoint.north_south * ((*angle as f64).to_radians().cos() as i64));
                    (
                        Waypoint {
                            north_south: y,
                            east_west: x,
                        },
                        ship.current_position,
                    )
                }
                Command::Right(angle) => {
                    let x = (ship.waypoint.east_west * ((*angle as f64).to_radians().cos() as i64))
                        + (ship.waypoint.north_south * ((*angle as f64).to_radians().sin() as i64));
                    let y = -(ship.waypoint.east_west
                        * ((*angle as f64).to_radians().sin() as i64))
                        + (ship.waypoint.north_south * ((*angle as f64).to_radians().cos() as i64));
                    (
                        Waypoint {
                            north_south: y,
                            east_west: x,
                        },
                        ship.current_position,
                    )
                }
                Command::Forward(value) => {
                    let north_south =
                        ship.current_position.north_south + ship.waypoint.north_south * value;
                    let east_west =
                        ship.current_position.east_west + ship.waypoint.east_west * value;
                    (
                        ship.waypoint,
                        Point {
                            north_south: north_south,
                            east_west: east_west,
                        },
                    )
                }
            };
            ship = ShipStar2 {
                waypoint: next_waypoint,
                current_position: next_position,
            };
        }
        let distance =
            ship.current_position.north_south.abs() + ship.current_position.east_west.abs();
        (ship, distance)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_day_12() {}

    #[test]
    fn check_day_12_star1() {}

    #[test]
    fn check_day_12_star2() {}

    #[test]
    fn check_day_12_star1_example_1_commands() {
        let file = read_file("./src/day_12/example.txt");
        let commands = input_to_commands(&file);
        assert_eq!(5, commands.len());
    }

    #[test]
    fn check_day_12_star1_input_commands() {
        let file = read_file("./src/day_12/input.txt");
        let commands = input_to_commands(&file);
        assert_eq!(768, commands.len());
    }

    #[test]
    fn check_day_12_star1_example_1_manhattan_distance() {
        let file = read_file("./src/day_12/example.txt");
        let commands = input_to_commands(&file);
        let ship = Ship::new();
        let (_, distance) = ship.manhattan_distance(commands);
        assert_eq!(25, distance);
    }

    #[test]
    fn check_day_12_star1_input_manhattan_distance() {
        let file = read_file("./src/day_12/input.txt");
        let commands = input_to_commands(&file);
        let ship = Ship::new();
        let (_, distance) = ship.manhattan_distance(commands);
        assert_eq!(759, distance);
    }

    #[test]
    fn check_day_12_star2_example_single_steps() {
        let ship = ShipStar2::new();
        let distance = ship.current_position.north_south + ship.current_position.east_west;
        assert_eq!(1, ship.waypoint.north_south);
        assert_eq!(10, ship.waypoint.east_west);
        assert_eq!(0, ship.current_position.north_south);
        assert_eq!(0, ship.current_position.east_west);
        assert_eq!(0, distance);
        let commands = vec![Command::Forward(10)];
        let (ship, distance) = ship.manhattan_distance(commands);
        assert_eq!(1, ship.waypoint.north_south);
        assert_eq!(10, ship.waypoint.east_west);
        assert_eq!(10, ship.current_position.north_south);
        assert_eq!(100, ship.current_position.east_west);
        assert_eq!(110, distance);
        let commands = vec![Command::North(3)];
        let (ship, distance) = ship.manhattan_distance(commands);
        assert_eq!(4, ship.waypoint.north_south);
        assert_eq!(10, ship.waypoint.east_west);
        assert_eq!(10, ship.current_position.north_south);
        assert_eq!(100, ship.current_position.east_west);
        assert_eq!(110, distance);
        let commands = vec![Command::Forward(7)];
        let (ship, distance) = ship.manhattan_distance(commands);
        assert_eq!(4, ship.waypoint.north_south);
        assert_eq!(10, ship.waypoint.east_west);
        assert_eq!(38, ship.current_position.north_south);
        assert_eq!(170, ship.current_position.east_west);
        assert_eq!(208, distance);
        let commands = vec![Command::Right(90)];
        let (ship, distance) = ship.manhattan_distance(commands);
        assert_eq!(-10, ship.waypoint.north_south);
        assert_eq!(4, ship.waypoint.east_west);
        assert_eq!(38, ship.current_position.north_south);
        assert_eq!(170, ship.current_position.east_west);
        assert_eq!(208, distance);
        let commands = vec![Command::Forward(11)];
        let (ship, distance) = ship.manhattan_distance(commands);
        assert_eq!(-10, ship.waypoint.north_south);
        assert_eq!(4, ship.waypoint.east_west);
        assert_eq!(-72, ship.current_position.north_south);
        assert_eq!(214, ship.current_position.east_west);
        assert_eq!(286, distance);
    }

    #[test]
    fn check_day_12_star2_example() {
        let file = read_file("./src/day_12/example.txt");
        let commands = input_to_commands(&file);
        let ship = ShipStar2::new();
        let (_, distance) = ship.manhattan_distance(commands);
        assert_eq!(286, distance);
    }

    #[test]
    fn check_day_12_star2_input() {
        let file = read_file("./src/day_12/input.txt");
        let commands = input_to_commands(&file);
        let ship = ShipStar2::new();
        let (_, distance) = ship.manhattan_distance(commands);
        assert_eq!(45763, distance);
    }
}
