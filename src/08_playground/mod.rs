struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Edge {
    a: usize,
    b: usize,
    d: i64,
}

struct JunctionBoxes {
    p: Vec<Point>,
}

fn dist(a: &Point, b: &Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;

    dx * dx + dy * dy + dz * dz
}

fn find_three_largest_circuits_part_1(junction_boxes: JunctionBoxes) -> usize {
    let p = junction_boxes.p;
    let n = p.len();

    let edges = {
        let mut v = unsafe { Box::new_uninit_slice(n * (n - 1) / 2).assume_init() };
        let mut i = 0;
        for a in 0..(n - 1) {
            for b in (a + 1)..n {
                v[i] = dist(&p[a], &p[b]);
                i += 1;
            }
        };
        v
    };

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
