use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            'S' => Ok(Tile::Start),
            '^' => Ok(Tile::Splitter),
            '|' => Ok(Tile::Beam),
            _ => Err("unexpected char {c}"),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Start => 'S',
            Tile::Splitter => '^',
            Tile::Beam => '|',
        }
    }
}

struct Map {
    w: usize,
    h: usize,
    data: Vec<Tile>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut lines = input.lines().peekable();

        // width from first line
        let w = lines.peek().unwrap().len();

        let data: Vec<Tile> = lines
            .by_ref()
            .flat_map(|line| line.chars().map(|c| Tile::try_from(c).unwrap()))
            .collect();

        let h = data.len() / w;

        Self { w, h, data }
    }

    fn at(&self, x: usize, y: usize) -> Option<&Tile> {
        let (x, y) = (x, y);
        if x >= self.w || y >= self.h {
            return None;
        }

        let index = x + self.w * y;
        Some(&self.data[index])
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        let index = x + self.w * y;
        self.data[index] = tile;
    }

    fn to_string(&self) -> String {
        let mut string = String::new();
        for chunk in self.data.as_slice().chunks(self.w) {
            for tile in chunk.iter() {
                string.push(Tile::into(*tile));
            }
            string.push('\n');
        }
        string
    }

    fn start_pos(&self) -> (usize, usize) {
        let pos = self
            .data
            .iter()
            .position(|t| matches!(t, Tile::Start))
            .unwrap();

        (pos % self.w, pos / self.w)
    }
}

fn find_tachyon_beams_part_1(input: &str) -> usize {
    let mut map = Map::new(input);

    let start = map.start_pos();
    draw_ray_recursive(start, &mut map);

    let mut answer = 0;
    for y in 0..map.h {
        for x in 0..map.w {
            if matches!(map.at(x, y), Some(Tile::Splitter))
                && matches!(map.at(x, y - 1), Some(Tile::Beam))
            {
                answer += 1;
            }
        }
    }

    answer
}

fn find_tachyon_beams_part_2(input: &str) -> u64 {
    let map = Map::new(input);
    let (sx, sy) = map.start_pos();

    let mut memo = HashMap::new();
    count_timelines_from(sx as isize, sy, &map, &mut memo)
}

fn count_timelines_from(
    x: isize,
    y: usize,
    map: &Map,
    memo: &mut HashMap<(isize, usize), u64>,
) -> u64 {
    if x < 0 || x >= map.w as isize {
        return 1;
    }

    let key = (x, y);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let ux = x as usize;

    for yy in (y + 1)..map.h {
        match map.at(ux, yy) {
            Some(Tile::Splitter) => {
                let left = count_timelines_from(x - 1, yy, map, memo);
                let right = count_timelines_from(x + 1, yy, map, memo);
                let total = left + right;
                memo.insert(key, total);
                return total;
            }
            _ => {}
        }
    }

    memo.insert(key, 1);
    1
}

fn draw_ray_recursive(from: (usize, usize), map: &mut Map) {
    // println!("{}", map.to_string());

    if let Some((x, y)) = draw_ray(from, map) {
        draw_ray_recursive((x - 1, y), map);
        draw_ray_recursive((x + 1, y), map);
    }
}

fn draw_ray(from: (usize, usize), map: &mut Map) -> Option<(usize, usize)> {
    let x = from.0;
    let mut y = from.1;

    loop {
        y += 1;

        match map.at(x, y) {
            Some(Tile::Empty) => {
                map.set(x, y, Tile::Beam);
            }
            Some(Tile::Splitter) => {
                return Some((x, y));
            }
            _ => {
                return None;
            }
        }
    }
}

pub fn part_1(input: &str) -> usize {
    find_tachyon_beams_part_1(input)
}

pub fn part_2(input: &str) -> u64 {
    find_tachyon_beams_part_2(input)
}

#[cfg(test)]
mod tests;
