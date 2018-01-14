use std::collections::HashMap;

#[allow(dead_code)]
fn day_6(mem: &str) -> Day6Result {
    let mut mem_banks = MemBanks::from_str(mem);
    let mut seen = HashMap::new();

    let mut cycles = 0;
    loop {
        if let Some(state_seen_after_num_cycles) = seen.get(&mem_banks) {
            return Day6Result {
                num_cycles: cycles,
                state_seen_after_num_cycles: cycles - *state_seen_after_num_cycles
            };
        }

        seen.insert(mem_banks.clone(), cycles);
        mem_banks = mem_banks.redistribute();
        cycles += 1;
    }
}

#[allow(dead_code)]
struct Day6Result {
    num_cycles: i64,
    state_seen_after_num_cycles: i64
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct MemBanks {
    mem: Vec<usize>
}

impl MemBanks {
    fn from_str(raw_mem: &str) -> MemBanks {
        let mem = raw_mem.split_whitespace().map(|s| s.parse().unwrap()).collect();
        MemBanks { mem: mem }
    }

    fn redistribute(&self) -> MemBanks {
        let (max_idx, max_val) = self.mem.iter().enumerate().fold((0, 0), |acc, (idx, &val)| {
            if val > acc.1 || ((val == acc.1) && idx < acc.0) {
                (idx, val)
            } else {
                acc
            }
        });

        let len = self.mem.len();
        let mut new_mem = self.mem.clone();
        new_mem[max_idx] = 0;

        for offset in 1..max_val + 1 {
            let idx = (max_idx + offset) % len;
            new_mem[idx] += 1;
        }

        MemBanks { mem: new_mem }
    }
}

#[cfg(test)]
mod tests {
    use day_6;

    #[test]
    fn day_6_part_1_examples() {
        assert_eq!(day_6("0 2 7 0").num_cycles, 5);
    }

    #[test]
    fn day_6_part_2_examples() {
        assert_eq!(day_6("0 2 7 0").state_seen_after_num_cycles, 4);
    }

    #[test]
    fn day_6_part_1_test_input() {
        assert_eq!(day_6(include_str!("input")).num_cycles, 5042);
    }

    #[test]
    fn day_6_part_2_test_input() {
        assert_eq!(day_6(include_str!("input")).state_seen_after_num_cycles, 1086);
    }
}
