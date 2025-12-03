struct Range {
    min: i64,
    max: i64,
}

fn find_invalid_part_1(range: Range) -> impl Iterator<Item = i64> {
    let a = (range.min..=range.max).filter(|x| {
        let chars = x.to_string().as_str().chars().collect::<Vec<char>>();
        if !chars.len().is_multiple_of(2) {
            return false;
        }

        // let result = (1..=(chars.len() / 2)).any(|i| {
        let result = ((chars.len() / 2)..=(chars.len() / 2)).any(|i| {
            let mut chunks = chars.chunks(i);
            let first = chunks.next().unwrap();

            let is_invalid = chunks.all(|chunk| *chunk == *first);

            is_invalid
        });

        result
    });

    a
}

fn find_invalid_part_2(range: Range) -> impl Iterator<Item = i64> {
    let a = (range.min..=range.max).filter(|x| {
        let chars = x.to_string().as_str().chars().collect::<Vec<char>>();

        let result = (1..=(chars.len() / 2)).any(|i| {
            let mut chunks = chars.chunks(i);
            let first = chunks.next().unwrap();

            let is_invalid = chunks.all(|chunk| *chunk == *first);

            is_invalid
        });

        result
    });

    a
}

fn parse_input(s: &str) -> impl Iterator<Item = Range> + '_ {
    s.split_terminator(',')
        .filter(|part| !part.is_empty()) // in case there's a trailing comma
        .map(|range_str| {
            let (min_str, max_str) = range_str
                .split_once('-')
                .expect("each range must contain exactly one '-'");

            Range {
                min: min_str.parse().unwrap(),
                max: max_str.parse().unwrap(),
            }
        })
}

pub fn part_1(str: &str) -> i64 {
    let input = parse_input(str);
    input.map(find_invalid_part_1).flatten().sum::<i64>()
}

pub fn part_2(str: &str) -> i64 {
    let input = parse_input(str);
    input.map(find_invalid_part_2).flatten().sum::<i64>()
}
#[cfg(test)]
mod tests;
