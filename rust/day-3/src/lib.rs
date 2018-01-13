use std::collections::HashMap;

#[allow(dead_code)]
fn day_3_part_1(index: i64) -> i64 {
    if index == 1 {
        return 0;
    }

    let layer = (((index as f64).sqrt() - 1.0) / 2.0).ceil() as i64;
    let distance_to_closest_corner = (index - 1) % (2 * layer);
    let distance_to_closest_middle = (distance_to_closest_corner - layer).abs();

    distance_to_closest_middle + layer
}

#[allow(dead_code)]
fn day_3_part_2(target: i64) -> i64 {
    let mut mem: HashMap<Coords, i64> = HashMap::new();
    let mut coords = Coords::zero();
    let mut dir = Direction::Right;
    let mut moves_before_turn = 1;

    mem.insert(coords, 1);

    loop {
        for _ in 0..2 {
            for _ in 0..moves_before_turn {
                coords = coords.next(dir);
                let value =
                    coords.neighbours().iter()
                        .map(|neighbour_coords| mem.get(neighbour_coords).cloned().unwrap_or(0))
                        .sum();

                if value > target {
                    return value;
                }

                mem.insert(coords, value);
            }

            dir = dir.next();
        }

        moves_before_turn += 1;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            &Direction::Right => Direction::Up,
            &Direction::Up => Direction::Left,
            &Direction::Left => Direction::Down,
            &Direction::Down => Direction::Right
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coords {
    x: i64,
    y: i64
}

impl Coords {
    fn zero() -> Coords {
        Coords { x: 0, y: 0 }
    }

    fn next(&self, dir: Direction) -> Coords {
        match dir {
            Direction::Right => Coords { x: self.x + 1, y: self.y },
            Direction::Up => Coords { x: self.x, y: self.y + 1 },
            Direction::Left => Coords { x: self.x - 1, y: self.y },
            Direction::Down => Coords { x: self.x, y: self.y - 1 }
        }
    }

    fn neighbours(&self) -> Vec<Coords> {
        vec![
            Coords { x: self.x - 1, y: self.y - 1 },
            Coords { x: self.x, y: self.y - 1 },
            Coords { x: self.x + 1, y: self.y - 1 },
            Coords { x: self.x - 1, y: self.y },
            Coords { x: self.x + 1, y: self.y },
            Coords { x: self.x - 1, y: self.y + 1 },
            Coords { x: self.x, y: self.y + 1 },
            Coords { x: self.x + 1, y: self.y + 1 }
        ]
    }
}

#[cfg(test)]
mod tests {
    use day_3_part_1;
    use day_3_part_2;

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3_part_1(1), 0);
        assert_eq!(day_3_part_1(12), 3);
        assert_eq!(day_3_part_1(23), 2);
        assert_eq!(day_3_part_1(1024), 31);
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3_part_2(1), 2);
        assert_eq!(day_3_part_2(2), 4);
        assert_eq!(day_3_part_2(4), 5);
        assert_eq!(day_3_part_2(5), 10);
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3_part_1(368078), 371);
    }

    #[test]
    fn day_3_part_2_test_input() {
        assert_eq!(day_3_part_2(368078), 369601);
    }
}
