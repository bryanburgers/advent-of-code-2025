use crate::{lookup::Lookup, range::Range};

mod lookup;
mod range;

fn main() {
    let input = include_str!("input.txt");
    let value = part_one(input);
    println!("{}", value);
}

fn part_one(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let mut ranges = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let range: Range = line.parse().unwrap();
        ranges.push(range);
    }
    let lookup = Lookup::from(ranges);

    let mut count = 0;
    for line in lines {
        let number = line.parse().unwrap();
        if lookup.contains(number) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        let value = part_one(input);
        assert_eq!(value, 3);
    }
}
