use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn final_frequency(input: &[i32]) -> i32 {
    input.into_iter().sum()
}

#[aoc(day1, part2)]
pub fn repeat_frequencies(input: &[i32]) -> i32 {
    let mut current_freq = 0;
    let mut freqs_seen = HashSet::new();
    freqs_seen.insert(0);

    loop {
        for i in input {
            current_freq += i;
            let new_insert = freqs_seen.insert(current_freq);
            if !new_insert {
                return current_freq;
            }
        }
    }
}
