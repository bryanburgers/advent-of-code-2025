use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let part_1 = part_1(input);
    println!("{}", part_1);
}

fn part_1(input: &str) -> usize {
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

struct Lock {
    inner: u16,
}

impl Lock {
    fn new() -> Self {
        Self { inner: 50 }
    }

    fn current(&self) -> u16 {
        self.inner
    }

    fn turn(&mut self, turn: Turn) {
        let add = match turn {
            Turn::Left(left) => {
                let mut left = -(left as i16);
                while left < 100 {
                    left += 100;
                }
                left as u16
            }
            Turn::Right(right) => right,
        };
        self.inner += add;
        self.inner %= 100;
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
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let out = part_1(input);
        assert_eq!(out, 3);
    }
}
