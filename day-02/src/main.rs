use silly_number::SillyNumber;
use std::str::FromStr;

mod silly_number;

fn main() {
    let input = include_str!("input.txt");
    let part_one = part_one(input);
    println!("{part_one}");
}

fn part_one(text: &str) -> usize {
    let mut sum = 0;
    let ranges = text.trim().split(",");
    for range in ranges {
        let range: Range = range.trim().parse().unwrap();
        for silly_number in range.silly_numbers() {
            sum += silly_number;
        }
    }
    sum
}

struct SillyNumberInfiniteIterator {
    next: SillyNumber,
}

impl SillyNumberInfiniteIterator {
    pub fn starting_with(silly_number: SillyNumber) -> Self {
        Self { next: silly_number }
    }
}

impl Iterator for SillyNumberInfiniteIterator {
    type Item = SillyNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next;
        self.next = self.next.successor();
        Some(n)
    }
}

#[derive(Copy, Clone)]
struct Range {
    from: usize,
    to: usize,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("-").ok_or_else(|| ())?;
        let from = from.parse().map_err(|_| ())?;
        let to = to.parse().map_err(|_| ())?;
        Ok(Range::new(from, to))
    }
}

impl Range {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
    pub fn silly_numbers(self) -> RangeSillyNumberIterator {
        RangeSillyNumberIterator::new(self.from, self.to)
    }
}

struct RangeSillyNumberIterator {
    inf: SillyNumberInfiniteIterator,
    stop: usize,
}

impl RangeSillyNumberIterator {
    pub fn new(from: usize, to: usize) -> Self {
        let next = SillyNumber::next(from);
        let inf = SillyNumberInfiniteIterator::starting_with(next);
        RangeSillyNumberIterator { inf, stop: to }
    }
}

impl Iterator for RangeSillyNumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.inf.next().unwrap();
        if v.value() > self.stop {
            return None;
        }
        Some(v.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silly_number_iterator() {
        let silly_number_iterator = SillyNumberInfiniteIterator::starting_with(SillyNumber::ZERO);
        let collected = silly_number_iterator
            .take(12)
            .map(|val| val.value())
            .collect::<Vec<_>>();
        assert_eq!(
            collected,
            [0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111]
        );
    }

    #[test]
    fn test_range() {
        let range = Range::new(11, 22);
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [11, 22]);

        let range: Range = "95-115".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [99]);
        let range: Range = "998-1012".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [1010]);
        let range: Range = "1188511880-1188511890".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [1188511885]);
        let range: Range = "222220-222224".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [222222]);
        let range: Range = "1698522-1698528".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, []);
        let range: Range = "446443-446449".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [446446]);
        let range: Range = "38593856-38593862".parse().unwrap();
        let numbers = range.silly_numbers().collect::<Vec<_>>();
        assert_eq!(numbers, [38593859]);
    }

    #[test]
    fn test_part_one() {
        let example = include_str!("example.txt");
        let output = part_one(example);
        assert_eq!(output, 1227775554);
    }
}
