#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

struct Line {
    a: Point,
    b: Point,
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
            Some(Point { x, y })
        })
        .collect()
}

fn rect_bounds(a: Point, b: Point) -> (i64, i64, i64, i64) {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);
    (min_x, max_x, min_y, max_y)
}

fn rect_area(a: Point, b: Point) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn on_edge(p: Point, a: Point, b: Point) -> bool {
    if a.x == b.x {
        // vertical
        let x = a.x;
        let (y0, y1) = (a.y.min(b.y), a.y.max(b.y));
        p.x == x && p.y >= y0 && p.y <= y1
    } else {
        // horizontal
        let y = a.y;
        let (x0, x1) = (a.x.min(b.x), a.x.max(b.x));
        p.y == y && p.x >= x0 && p.x <= x1
    }
}

/// Even-odd rule for axis-aligned simple polygon.
/// Returns true if p is inside OR on the boundary.
fn point_inside_or_on(p: Point, edges: &[Line]) -> bool {
    // boundary check
    for e in edges {
        if on_edge(p, e.a, e.b) {
            return true;
        }
    }

    // ray cast to +inf in x; only vertical edges contribute
    let mut inside = false;
    for e in edges {
        if e.a.x == e.b.x {
            let x = e.a.x;
            let (y0, y1) = (e.a.y.min(e.b.y), e.a.y.max(e.b.y));
            // half-open to avoid double counting vertices
            if p.y >= y0 && p.y < y1 && p.x < x {
                inside = !inside;
            }
        }
    }
    inside
}

/// True if this boundary segment passes through the rectangle *interior*.
/// Touching rectangle border (including lying along it) is allowed.
fn edge_hits_rect_interior(e: &Line, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> bool {
    if e.a.x == e.b.x {
        // vertical segment at x = sx spanning y0..y1
        let sx = e.a.x;
        let (y0, y1) = (e.a.y.min(e.b.y), e.a.y.max(e.b.y));

        // must be strictly inside in x, and overlap interior in y
        if sx <= min_x || sx >= max_x {
            return false;
        }
        y0 < max_y && y1 > min_y
    } else {
        // horizontal segment at y = sy spanning x0..x1
        let sy = e.a.y;
        let (x0, x1) = (e.a.x.min(e.b.x), e.a.x.max(e.b.x));

        // must be strictly inside in y, and overlap interior in x
        if sy <= min_y || sy >= max_y {
            return false;
        }
        x0 < max_x && x1 > min_x
    }
}

fn rect_ok(a: Point, b: Point, edges: &[Line]) -> bool {
    let (min_x, max_x, min_y, max_y) = rect_bounds(a, b);

    // 1) boundary must not cut through rectangle interior
    if edges
        .iter()
        .any(|e| edge_hits_rect_interior(e, min_x, max_x, min_y, max_y))
    {
        return false;
    }

    // 2) if rectangle has an interior, a point inside it must be inside polygon
    if min_x < max_x && min_y < max_y {
        let probe = Point {
            x: min_x + 1,
            y: min_y + 1,
        };
        if !point_inside_or_on(probe, edges) {
            return false;
        }
    }

    true
}

pub fn part_1(input: &str) -> u64 {
    let p = parse_points(input);

    let mut best = 0u64;
    for a in 0..p.len().saturating_sub(1) {
        for b in (a + 1)..p.len() {
            best = best.max(rect_area(p[a], p[b]));
        }
    }
    best
}

pub fn part_2(input: &str) -> u64 {
    let p = parse_points(input);

    // build boundary edges (no clones needed if Point is Copy)
    let mut edges = Vec::with_capacity(p.len());
    for w in p.windows(2) {
        edges.push(Line { a: w[0], b: w[1] });
    }
    edges.push(Line {
        a: p[p.len() - 1],
        b: p[0],
    });

    let mut best = 0u64;

    for a in 0..p.len().saturating_sub(1) {
        for b in (a + 1)..p.len() {
            let area = rect_area(p[a], p[b]);
            if area <= best {
                continue;
            }
            if rect_ok(p[a], p[b], &edges) {
                best = area;
            }
        }
    }

    best
}
