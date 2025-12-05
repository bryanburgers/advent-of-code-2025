use std::str::FromStr;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Range {
    pub from: u64,
    pub to: u64,
}

impl Range {
    pub fn new(from: u64, to: u64) -> Self {
        assert!(from <= to);

        Range { from, to }
    }

    pub fn contains(&self, n: u64) -> bool {
        self.from <= n && n <= self.to
    }

    pub fn combine(self, other: Self) -> Option<Self> {
        if self.from > other.from {
            return other.combine(self);
        }

        if (self.to + 1) >= other.from {
            let to = std::cmp::max(self.to, other.to);
            Some(Self::new(self.from, to))
        } else {
            None
        }
    }

    pub fn size(&self) -> u64 {
        self.to - self.from + 1
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or("invalid range")?;
        let from = from.parse().map_err(|_| "invalid range")?;
        let to = to.parse().map_err(|_| "invalid range")?;
        Ok(Self::new(from, to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let range = Range::new(100, 200);
        assert!(!range.contains(99));
        assert!(range.contains(100));
        assert!(range.contains(101));
        assert!(range.contains(199));
        assert!(range.contains(200));
        assert!(!range.contains(201));
    }

    #[test]
    fn test_combine() {
        let output = Range::new(100, 200).combine(Range::new(150, 250));
        assert_eq!(output, Some(Range::new(100, 250)));

        let output = Range::new(150, 250).combine(Range::new(100, 200));
        assert_eq!(output, Some(Range::new(100, 250)));

        let output = Range::new(100, 200).combine(Range::new(250, 350));
        assert_eq!(output, None);

        let output = Range::new(100, 200).combine(Range::new(110, 190));
        assert_eq!(output, Some(Range::new(100, 200)));

        let output = Range::new(100, 200).combine(Range::new(100, 190));
        assert_eq!(output, Some(Range::new(100, 200)));

        let output = Range::new(110, 200).combine(Range::new(100, 200));
        assert_eq!(output, Some(Range::new(100, 200)));

        let output = Range::new(1, 4).combine(Range::new(5, 7));
        assert_eq!(output, Some(Range::new(1, 7)));

        let output = Range::new(5, 7).combine(Range::new(1, 4));
        assert_eq!(output, Some(Range::new(1, 7)));

        let output = Range::new(1, 3).combine(Range::new(5, 7));
        assert_eq!(output, None);
    }

    #[test]
    fn test_parse() {
        let parsed: Range = "1-10".parse().expect("successful parse");
        assert_eq!(parsed, Range::new(1, 10));
    }

    #[test]
    fn test_size() {
        let range = Range::new(1, 1);
        assert_eq!(range.size(), 1);

        let range = Range::new(1, 2);
        assert_eq!(range.size(), 2);

        let range = Range::new(100, 200);
        assert_eq!(range.size(), 101);
    }
}
