struct Point {
    x: u64,
    y: u64,
    z: u64,
}

fn dist(p1: &Point, p2: &Point) -> u64 {
    (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y) + (p1.z - p2.z) * (p1.z - p2.z)
}

struct JunctionBoxes {
    p: Vec<Point>,
}
//    0 1 2 3 4 x
// 0  x 0 1 2 3
// 1  x x 4 5 6
// 2  x x x 7 8
// 3  x x x x 9
// 4  x x x x x
// y
// (x,y)
// 0,1 -> 0
// 0,2 -> 1
// 0,3 -> 2
// 0,4 -> 3
//
// 1,2 -> 3
// 1,3 -> 4
//
// 2,3 -> 5

fn at(x: usize, y: usize, n: usize, matrix: &[u64]) -> u64 {
    if x == y {
        return 0;
    }

    let (low, high) = if y < x { (y, x) } else { (x, y) };

    matrix[idx(low, high, n)]
}

fn idx(low: usize, high: usize, n: usize) -> usize {
    low * (2 * n - low - 1) / 2 + (high - low - 1)
}

fn create_matrix(p: &Vec<Point>) -> Vec<u64> {
    let n = p.len();
    let mut matrix = vec![0; n * (n - 1) / 2];

    for y in 0..(n - 1) {
        for x in (y + 1)..n {
            matrix[idx(y, x, n)] = dist(&p[x], &p[y]);
        }
    }

    matrix
}

fn closest_pair(matrix: &Vec<u64>) -> (usize,usize) {
    let mut closest_pair = (0,0);
    let mut distance = u64::MAX;

    for y in 0..(n - 1) {
        for x in (y + 1)..n {
            if matrix[idx(y, x, n)] < distance {

            }
        }
    }

    0
}

fn find_three_largest_circuits_part_1(mut junction_boxes: JunctionBoxes) -> usize {
    // let p = junction_boxes.p;

    let p = junction_boxes.p;
    let matrix = create_matrix(&p);

    let min_index = matrix
        .iter()
        .enumerate()
        .min_by_key(|&(_, v)| v)
        .map(|(i, _)| i);

    0
}

fn find_three_largest_circuits_part_2(junction_boxes: JunctionBoxes) -> usize {
    0
}

fn parse_input(input: &str) -> JunctionBoxes {
    JunctionBoxes {
        p: input
            .lines()
            .map(|line| {
                let mut numbers = line.split_terminator(',').map(|n| n.parse().unwrap());
                Point {
                    x: numbers.next().unwrap(),
                    y: numbers.next().unwrap(),
                    z: numbers.next().unwrap(),
                }
            })
            .collect(),
    }
}

pub fn part_1(input: &str) -> usize {
    let junction_boxes = parse_input(input);
    find_three_largest_circuits_part_1(junction_boxes)
}

pub fn part_2(input: &str) -> usize {
    let junction_boxes = parse_input(input);
    find_three_largest_circuits_part_2(junction_boxes)
}

#[cfg(test)]
mod tests;
