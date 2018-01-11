#[allow(dead_code)]
fn day_2_part_1(input: &str) -> i64 {
    input.lines().map(|row| {
        let state = row.split_whitespace().fold(ChecksumState::zero(), |acc, c| {
            let incoming: i64 = c.parse().unwrap();
            acc.update(incoming)
        });

        state.largest.unwrap_or(0) - state.smallest.unwrap_or(0)
    }).sum()
}

#[allow(dead_code)]
fn day_2_part_2(input: &str) -> i64 {
    input.lines().map(|row| {
        let columns: Vec<i64> = row.split_whitespace().map(|c| c.parse().unwrap()).collect();

        let mut result: i64 = 0;

        'outer: for (i, x) in columns.iter().enumerate() {
            for (j, y) in columns.iter().enumerate() {
                if i == j {
                    continue;
                }

                let xm = std::cmp::max(x, y);
                let ym = std::cmp::min(x, y);

                if xm % ym == 0 {
                    result = xm / ym;
                    break 'outer;
                }
            }
        }

        result
    }).sum()
}

struct ChecksumState {
    largest: Option<i64>,
    smallest: Option<i64>
}

impl ChecksumState {
    fn zero() -> ChecksumState {
        ChecksumState {
            largest: None,
            smallest: None
        }
    }

    fn update(&self, incoming: i64) -> ChecksumState {
        let largest = match self.largest {
            None => incoming,
            Some(curr) => {
                if incoming > curr {
                    incoming
                } else {
                    curr
                }
            }
        };

        let smallest = match self.smallest {
            None => incoming,
            Some(curr) => {
                if incoming < curr {
                    incoming
                } else {
                    curr
                }
            }
        };

        ChecksumState {
            largest: Some(largest),
            smallest: Some(smallest)
        }
    }
}

#[cfg(test)]
mod tests {
    use day_2_part_1;
    use day_2_part_2;

    #[test]
    fn day_2_part_1_examples() {
        assert_eq!(day_2_part_1("5 1 9 5\n7 5 3\n2 4 6 8"), 18);
    }

    #[test]
    fn day_2_part_2_examples() {
        assert_eq!(day_2_part_2("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9);
    }

    const INPUT: &'static str = include_str!("input");

    #[test]
    fn day_2_part_1_test_input() {
        assert_eq!(day_2_part_1(INPUT), 45158);
    }

    #[test]
    fn day_2_part_2_test_input() {
        assert_eq!(day_2_part_2(INPUT), 294);
    }
}
