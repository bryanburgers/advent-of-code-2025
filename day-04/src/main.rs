fn main() {
    let input = include_str!("input.txt");
    let value = part_one(input);
    println!("{value}");
    let value = part_two(input);
    println!("{value}");
}

fn part_one(str: &str) -> usize {
    let grid = Grid::parse(str);
    let mut count = 0;
    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize {
            let mut neighbor_count = 0;

            for (nx, ny) in [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ] {
                if grid.get(nx, ny) == Some(Entry::Paper) {
                    neighbor_count += 1;
                }
            }

            if grid.get(x, y) == Some(Entry::Paper) && neighbor_count < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part_two(str: &str) -> usize {
    let mut grid = Grid::parse(str);
    let mut removed = 0;
    loop {
        let mut removed_this_time = 0;
        for y in 0..grid.height as isize {
            for x in 0..grid.width as isize {
                let mut neighbor_count = 0;

                for (nx, ny) in [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ] {
                    if grid.get(nx, ny) == Some(Entry::Paper) {
                        neighbor_count += 1;
                    }
                }

                if grid.get(x, y) == Some(Entry::Paper) && neighbor_count < 4 {
                    removed_this_time += 1;
                    grid.remove(x, y);
                }
            }
        }

        if removed_this_time > 0 {
            removed += removed_this_time;
        } else {
            break;
        }
    }

    removed
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<Entry>>,
}

impl Grid {
    pub fn parse(str: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut data = Vec::new();
        let lines = str.trim().lines();
        for line in lines {
            let mut row = Vec::new();
            for byte in line.trim().as_bytes() {
                let entry = match byte {
                    b'.' => Entry::Empty,
                    b'@' => Entry::Paper,
                    _ => panic!("unexpected byte"),
                };
                row.push(entry);
            }
            width = row.len();
            data.push(row);
        }

        height = data.len();

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<Entry> {
        if x < 0 {
            return None;
        }
        if y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= self.height {
            return None;
        }
        if x >= self.width {
            return None;
        }
        Some(self.data[y][x])
    }

    pub fn remove(&mut self, x: isize, y: isize) {
        if x < 0 {
            return;
        }
        if y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= self.height {
            return;
        }
        if x >= self.width {
            return;
        }
        self.data[y][x] = Entry::Empty;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Entry {
    Empty,
    Paper,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        let value = part_one(input);
        assert_eq!(value, 13);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("example.txt");
        let value = part_two(input);
        assert_eq!(value, 43);
    }
}
