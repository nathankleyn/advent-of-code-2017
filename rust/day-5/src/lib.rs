#[allow(dead_code)]
fn day_5(raw_instructions: &str, penalise_large_numbers: bool) -> i64 {
    let mut instructions: Vec<i64> = raw_instructions.lines().map(|s| s.parse().unwrap()).collect();
    let len = instructions.len();

    if len == 0 {
        return 0;
    }

    let mut ptr = 0;
    let mut jumps = 0;

    loop {
        let offset = instructions[ptr];
        let next_ptr = ((ptr as i64) + offset) as usize;
        if penalise_large_numbers && offset >= 3 {
            instructions[ptr] -= 1;
        } else {
            instructions[ptr] += 1;
        }
        jumps += 1;

        if next_ptr >= len {
            return jumps;
        }

        ptr = next_ptr;
    }
}

#[cfg(test)]
mod tests {
    use day_5;

    #[test]
    fn day_5_part_1_examples() {
        assert_eq!(day_5("0\n3\n0\n1\n-3", false), 5);
    }

    #[test]
    fn day_5_part_2_examples() {
        assert_eq!(day_5("0\n3\n0\n1\n-3", true), 10);
    }

    #[test]
    fn day_5_part_1_test_input() {
        assert_eq!(day_5(include_str!("input"), false), 342669);
    }

    #[test]
    fn day_5_part_2_test_input() {
        assert_eq!(day_5(include_str!("input"), true), 25136209);
    }
}
