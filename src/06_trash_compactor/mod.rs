// second part it completely AI generated
// Was hoping for an interesting algorithmic challenge instead of just a parsing challenge 

use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

impl TryFrom<&str> for Operator {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Operator::Addition),
            "*" => Ok(Operator::Multiplication),
            _ => Err("invalid operator"),
        }
    }
}

struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(self) -> u64 {
        match self.operator {
            Operator::Addition => self.numbers.into_iter().sum(),
            Operator::Multiplication => self.numbers.into_iter().product(),
        }
    }
}

struct Homework<I>
where
    I: Iterator<Item = Problem>,
{
    problems: I,
}

// ----------------- PART 1: row-wise interpretation -----------------

fn find_fresh_ingredient_part_1<I>(homework: Homework<I>) -> u64
where
    I: Iterator<Item = Problem>,
{
    homework.problems.map(Problem::solve).sum()
}

fn parse_input(input: &str) -> Homework<impl Iterator<Item = Problem> + '_> {
    // All lines, last one is the operator row
    let lines: Vec<&str> = input.lines().collect();
    let n_lines = lines.len();
    assert!(n_lines >= 2, "expected at least one number row and one operator row");

    let operator_line = lines[n_lines - 1];
    let number_lines = &lines[..n_lines - 1];

    // Parse numbers row-wise (this is your original interpretation)
    let number_rows: Vec<Vec<u64>> = number_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let operators: Vec<Operator> = operator_line
        .split_whitespace()
        .map(|op| op.try_into().unwrap())
        .collect();

    let cols = number_rows[0].len();
    let rows = number_rows.len();

    let mut problems = Vec::with_capacity(cols);
    for col in 0..cols {
        let mut nums = Vec::with_capacity(rows);
        for row in 0..rows {
            nums.push(number_rows[row][col]);
        }

        problems.push(Problem {
            numbers: nums,
            operator: operators[col],
        });
    }

    Homework {
        problems: problems.into_iter(),
    }
}

// ----------------- PART 2: column-wise interpretation -----------------

fn find_fresh_ingredient_part_2<I>(homework: Homework<I>) -> u64
where
    I: Iterator<Item = Problem>,
{
    homework.problems.map(Problem::solve).sum()
}

fn parse_input_part_2(input: &str) -> Homework<impl Iterator<Item = Problem> + '_> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    assert!(height >= 2, "need at least one number row plus operator row");

    // Build rectangular char grid, right-padded with spaces
    let width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut row: Vec<char> = line.chars().collect();
            row.resize(width, ' ');
            row
        })
        .collect();

    let num_rows = height - 1; // last row = operators

    // Find separator columns: columns that are spaces in *all* rows
    let mut sep_cols = Vec::new();
    for x in 0..width {
        let mut all_space = true;
        for y in 0..height {
            if grid[y][x] != ' ' {
                all_space = false;
                break;
            }
        }
        if all_space {
            sep_cols.push(x);
        }
    }

    // Split into problem column ranges [start, end)
    let mut ranges = Vec::new();
    let mut start = 0usize;
    for &sep in &sep_cols {
        if start < sep {
            ranges.push((start, sep));
        }
        start = sep + 1;
    }
    if start < width {
        ranges.push((start, width));
    }

    let mut problems = Vec::new();

    for (start_col, end_col) in ranges {
        // Find operator for this problem in the bottom row
        let op_char = (start_col..end_col)
            .filter_map(|x| {
                let c = grid[num_rows][x];
                if c == '+' || c == '*' {
                    Some(c)
                } else {
                    None
                }
            })
            .next()
            .expect("no operator found in problem");

        let operator = match op_char {
            '+' => Operator::Addition,
            '*' => Operator::Multiplication,
            _ => unreachable!(),
        };

        // Build numbers column-wise, RIGHT-TO-LEFT, top-to-bottom digits
        let mut numbers = Vec::new();
        for x in (start_col..end_col).rev() {
            let mut num_str = String::new();
            for y in 0..num_rows {
                let c = grid[y][x];
                if c.is_ascii_digit() {
                    num_str.push(c);
                }
            }
            if !num_str.is_empty() {
                let value = num_str.parse::<u64>().unwrap();
                numbers.push(value);
            }
        }

        problems.push(Problem { numbers, operator });
    }

    Homework {
        problems: problems.into_iter(),
    }
}

// ----------------- Public entrypoints -----------------

pub fn part_1(input: &str) -> u64 {
    let homework = parse_input(input);
    find_fresh_ingredient_part_1(homework)
}

pub fn part_2(input: &str) -> u64 {
    let homework = parse_input_part_2(input);
    find_fresh_ingredient_part_2(homework)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part2() {
        let input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_2(input), 3_263_827);
    }
}
