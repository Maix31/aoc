#[derive(Debug)]
enum Tile {
    Empty,
    ToiletPaper,
    Accessible,
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
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '@' => Tile::ToiletPaper,
                    '.' => Tile::Empty,
                    _ => panic!("unexpected char {c}"),
                })
            })
            .collect();

        let h = data.len() / w;

        Self { w, h, data }
    }

    fn at(&self, x: i64, y: i64) -> Option<&Tile> {
        // guard against negative and out-of-bounds coords
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.w || y >= self.h {
            return None;
        }

        let index = x + self.w * y;
        Some(&self.data[index])
    }

    fn set_accessible(&mut self, x: usize, y: usize) {
        let index = x + self.w * y;
        self.data[index] = Tile::Accessible;
    }

    fn to_string(&self) -> String{
        let mut string = String::new();
        for chunk in self.data.as_slice().chunks(self.w) {
            for tile in chunk.iter() {
                string.push(
                match tile {
                    Tile::Empty => '.',
                    Tile::ToiletPaper => '@',
                    Tile::Accessible => 'x',
                });
            }
            string.push('\n');
        }
        string
    }

    fn neighbours(&self, x: usize, y: usize) -> [Option<&Tile>; 8] {
        let x = x as i64;
        let y = y as i64;
        [
            self.at(x - 1, y - 1),
            self.at(x,     y - 1),
            self.at(x + 1, y - 1),
            self.at(x - 1, y),
            self.at(x + 1, y),
            self.at(x - 1, y + 1),
            self.at(x,     y + 1),
            self.at(x + 1, y + 1),
        ]
    }

    fn can_access(&self, x: usize, y: usize) -> bool {
        // neighbours() returns [Option<&Tile>; 8]
        // into_iter() -> Iterator<Item = Option<&Tile>>
        self.neighbours(x, y)
            .into_iter()
            .filter(|t| matches!(t, Some(Tile::ToiletPaper)))
            .count()
            < 5
    }
}

fn find_printing_department_part_1(input: &str) -> usize {
    let map = Map::new(input);
    let mut new_map = Map::new(input);
    let result = (0..map.h)
        .map(|y| (0..map.w).filter(|&x| {
            let can_access = map.can_access(x, y);
            if can_access {
                new_map.set_accessible(x, y);
            }
            can_access
        }).count())
        .sum();

    print!("{}", map.to_string());

    result
}

fn find_printing_department_part_2(_input: &str) -> u64 {
    0
}

pub fn part_1(input: &str) -> usize {
    find_printing_department_part_1(input)
}

pub fn part_2(input: &str) -> u64 {
    find_printing_department_part_2(input)
}

#[cfg(test)]
mod tests;
