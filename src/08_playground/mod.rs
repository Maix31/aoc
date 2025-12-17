use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn dist2(a: &Point, b: &Point) -> u128 {
    let dx = (a.x as i128) - (b.x as i128);
    let dy = (a.y as i128) - (b.y as i128);
    let dz = (a.z as i128) - (b.z as i128);
    (dx * dx + dy * dy + dz * dz) as u128
}

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let root = self.find(self.parent[x]);
        self.parent[x] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.components -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        for i in 0..self.parent.len() {
            let _ = self.find(i);
        }
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut it = line.split_terminator(',');
            let x: i64 = it.next()?.trim().parse().ok()?;
            let y: i64 = it.next()?.trim().parse().ok()?;
            let z: i64 = it.next()?.trim().parse().ok()?;
            Some(Point { x, y, z })
        })
        .collect()
}

/// Part 1 helper: take K smallest edges by (d,a,b) without storing all edges.
fn k_smallest_edges(points: &[Point], k: usize) -> Vec<(u128, usize, usize)> {
    let n = points.len();
    if n < 2 || k == 0 {
        return Vec::new();
    }
    let total = n * (n - 1) / 2;
    let k = k.min(total);

    // keep the best K edges; heap top is the worst among kept
    let mut heap: BinaryHeap<(u128, usize, usize)> = BinaryHeap::with_capacity(k);

    for a in 0..(n - 1) {
        for b in (a + 1)..n {
            let edge = (dist2(&points[a], &points[b]), a, b);

            if heap.len() < k {
                heap.push(edge);
            } else {
                let worst = *heap.peek().unwrap();
                if edge < worst {
                    heap.pop();
                    heap.push(edge);
                }
            }
        }
    }

    let mut edges = heap.into_vec();
    edges.sort_unstable(); // ascending
    edges
}

fn solve_part1(points: &[Point], k: usize) -> u64 {
    let n = points.len();
    if n == 0 {
        return 0;
    }

    let edges = k_smallest_edges(points, k);

    let mut dsu = Dsu::new(n);
    for &(_d, a, b) in &edges {
        dsu.union(a, b);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let a = sizes.get(0).copied().unwrap_or(1) as u64;
    let b = sizes.get(1).copied().unwrap_or(1) as u64;
    let c = sizes.get(2).copied().unwrap_or(1) as u64;
    a * b * c
}

/// Part 2: build an MST with Prim (O(n^2)), track the maximum edge used in the MST.
/// That max edge is the “last edge you need” under closest-first union (when weights are unique).
fn solve_part2(points: &[Point]) -> u64 {
    let n = points.len();
    if n <= 1 {
        return 0;
    }

    let mut in_tree = vec![false; n];
    let mut best_dist = vec![u128::MAX; n];
    let mut best_parent = vec![usize::MAX; n];

    // start from 0
    in_tree[0] = true;
    for v in 1..n {
        best_dist[v] = dist2(&points[0], &points[v]);
        best_parent[v] = 0;
    }

    let mut max_edge = (0usize, 0usize);
    let mut max_w: u128 = 0;

    for _ in 1..n {
        // pick next vertex with minimal best_dist
        let mut u = usize::MAX;
        let mut u_w = u128::MAX;

        for v in 0..n {
            if !in_tree[v] && best_dist[v] < u_w {
                u_w = best_dist[v];
                u = v;
            }
        }

        // should not happen in a complete graph
        if u == usize::MAX {
            break;
        }

        let p = best_parent[u];
        in_tree[u] = true;

        // edge (p,u) enters MST
        if u_w > max_w {
            max_w = u_w;
            max_edge = (p, u);
        }

        // update distances to the tree using newly added u
        for v in 0..n {
            if !in_tree[v] {
                let d = dist2(&points[u], &points[v]);
                if d < best_dist[v] {
                    best_dist[v] = d;
                    best_parent[v] = u;
                }
            }
        }
    }

    let (a, b) = max_edge;
    (points[a].x as i128 * points[b].x as i128) as u64
}

pub fn part_1(input: &str) -> u64 {
    let points = parse_points(input);
    solve_part1(&points, 1000)
}

pub fn part_2(input: &str) -> u64 {
    let points = parse_points(input);
    solve_part2(&points)
}
