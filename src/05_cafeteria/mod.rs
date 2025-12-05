struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn overlap(&self, other: &Range) -> bool {
        let a = self;
        let b = other;

        ((a.min <= b.min && b.min <= a.max) || (a.min <= b.max && b.max <= a.max))
            || ((b.min <= a.min && a.min <= b.max) || (b.min <= a.max && a.max <= b.max))
    }

    fn combine(&self, other: &Range) -> Range {
        Range {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    fn size(&self) -> u64 {
        self.max - self.min + 1
    }

    fn new(min: u64, max: u64) -> Self {
        assert!(min <= max);
        Range { min, max }
    }
}

#[cfg(test)]
mod tests_range {
    use crate::cafeteria::Range;

    #[test]
    fn test_overlap() {
        assert_eq!(Range::new(0, 1).overlap(&Range::new(2, 3)), false);
        assert_eq!(Range::new(0, 2).overlap(&Range::new(2, 3)), true);
        assert_eq!(Range::new(3, 4).overlap(&Range::new(2, 3)), true);
        assert_eq!(Range::new(4, 5).overlap(&Range::new(2, 3)), false);

        assert_eq!(Range::new(2, 3).overlap(&Range::new(0, 1)), false);
        assert_eq!(Range::new(2, 3).overlap(&Range::new(0, 2)), true);
        assert_eq!(Range::new(2, 3).overlap(&Range::new(3, 4)), true);
        assert_eq!(Range::new(2, 3).overlap(&Range::new(4, 5)), false);

        assert_eq!(Range::new(2, 3).overlap(&Range::new(2, 3)), true);
    }
}

struct Database {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

fn find_fresh_ingredient_part_1(database: Database) -> usize {
    database
        .ids
        .iter()
        .filter(|id| {
            database
                .ranges
                .iter()
                .any(|range| range.min <= **id && **id <= range.max)
        })
        .count()
}

fn find_fresh_ingredient_part_2(database: Database) -> usize {
    let mut ranges = database.ranges;

    loop {
        let mut any_overlap = false;
        ranges.sort_by(|l, r| l.min.cmp(&r.min));

        let mut i = 0;
        loop {
            if i > (ranges.len() - 2) {
                break;
            }

            let mut j = i;
            loop {
                if j > (ranges.len() - 2) {
                    break;
                }
                let a = &ranges[i];
                let b = &ranges[j + 1];

                if a.overlap(b) {
                    let new = a.combine(b);
                    ranges.remove(i);
                    ranges.remove(j);
                    ranges.insert(i, new);
                    any_overlap = true;
                } else {
                    j = j + 1
                }
            }

            i += 1;
        }

        if !any_overlap {
            break;
        }
    }

    ranges.iter().map(|r| r.size()).sum::<u64>() as usize
}

fn parse_input(input: &str) -> Database {
    let split_pos = input
        .lines()
        .into_iter()
        .position(|line| line.is_empty())
        .unwrap();

    let mut lines = input.lines();
    let ranges = (0..split_pos)
        .map(|_| {
            let line = lines.next().unwrap();
            let mut split = line.split_terminator('-');
            let min = split.next().unwrap().parse().unwrap();
            let max = split.next().unwrap().parse().unwrap();
            Range { min, max }
        })
        .collect::<Vec<Range>>();

    assert!(lines.next().unwrap().is_empty());

    let ids = lines.map(|str| str.parse().unwrap()).collect();

    Database { ranges, ids }
}

pub fn part_1(input: &str) -> usize {
    let database = parse_input(input);
    find_fresh_ingredient_part_1(database)
}

pub fn part_2(input: &str) -> usize {
    let database = parse_input(input);
    find_fresh_ingredient_part_2(database)
}

#[cfg(test)]
mod tests;
