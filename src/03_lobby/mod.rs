use std::collections::HashMap;
fn find_max_joltage_part_1(str: &str) -> u64 {
    let digits = str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut copy = digits.to_owned();
    let arr = copy.as_mut_slice();

    for i in (1..arr.len()).rev() {
        arr[i - 1] = arr[i].max(arr[i - 1]);
    }

    // println!("{:?}", digits);
    // println!("{:?}", arr);
    // println!();

    let mut a = u64::MIN;
    for i in 0..(arr.len() - 1) {
        a = a.max(digits[i] as u64 * 10 + arr[i + 1] as u64);
    }

    a
}

fn find_max_joltage_part_2<'a>(str: &'a str , n: u32, map: &mut HashMap::<(&'a str, u32), u64>) -> u64 {
    if n == 2 {
        return find_max_joltage_part_1(str);
    }

    let digits = str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut copy = digits.to_owned();
    let arr = copy.as_mut_slice();

    let mut a = u64::MIN;
    for i in 0..=(arr.len() - n as usize) {
        let key = (&str[i + 1..], n - 1);
        let cached_value = if let Some(cached_value) = map.get(&key) {
            *cached_value
        } else {
            let result = find_max_joltage_part_2(&str[i + 1..], n - 1, map);
            map.insert(key, result);
            result
        };
        
        a = a.max(
            (digits[i] as u64) * 10u64.pow(n - 1) + cached_value,
        );
    }

    a
}

fn parse_input(s: &str) -> impl Iterator<Item = &str> + '_ {
    s.split_whitespace()
}

pub fn part_1(str: &str) -> u64 {
    let input = parse_input(str);
    input.map(find_max_joltage_part_1).sum()
}

pub fn part_2(str: &str) -> u64 {
    let input = parse_input(str);
    input
        .map(|s| {
            let mut map = HashMap::<(&str, u32), u64>::new();
            find_max_joltage_part_2(s, 12, &mut map)
        })
        .sum()
}
#[cfg(test)]
mod tests;
