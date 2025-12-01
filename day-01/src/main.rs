use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let part_1 = part_1(input);
    println!("{}", part_1);
    let part_2 = part_2(input);
    println!("{}", part_2);
}

fn part_1(input: &str) -> u16 {
    let mut turns = Vec::new();
    for line in input.lines() {
        let turn: Turn = line.parse().expect("Expected parse");
        turns.push(turn);
    }

    let mut lock = Lock::new();
    let mut count = 0;

    for turn in turns {
        lock.turn(turn);
        if lock.current() == 0 {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str) -> u16 {
    let mut turns = Vec::new();
    for line in input.lines() {
        let turn: Turn = line.parse().expect("Expected parse");
        turns.push(turn);
    }

    let mut lock = Lock::new();

    for turn in turns {
        lock.turn(turn);
    }

    lock.clicks()
}

struct Lock {
    current: u16,
    clicks: u16,
}

impl Lock {
    fn new() -> Self {
        Self {
            current: 50,
            clicks: 0,
        }
    }

    fn current(&self) -> u16 {
        self.current
    }

    fn clicks(&self) -> u16 {
        self.clicks
    }

    fn turn(&mut self, turn: Turn) {
        let mut val = self.current as i16;
        let mut clicks = 0;

        match turn {
            Turn::Left(left) => {
                val = val - left as i16;
                while val < 0 {
                    val += 100;
                    clicks += 1;
                }
            }
            Turn::Right(right) => {
                val = val + right as i16;
                while val >= 100 {
                    val -= 100;
                    clicks += 1;
                }
            }
        }

        self.current = val as u16;
        self.clicks += clicks;
    }
}

#[derive(Copy, Clone)]
enum Turn {
    Left(u16),
    Right(u16),
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(left) = s.strip_prefix("L") {
            let left = left.parse().map_err(|_| ())?;
            Ok(Turn::Left(left))
        } else if let Some(right) = s.strip_prefix("R") {
            let right = right.parse().map_err(|_| ())?;
            Ok(Turn::Right(right))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let out = part_1(input);
        assert_eq!(out, 3);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let out = part_2(input);
        assert_eq!(out, 6);
    }
}
