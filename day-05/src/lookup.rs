use crate::range::Range;

pub struct Lookup {
    ranges: Box<[Range]>,
}

impl Lookup {
    pub fn from(ranges: impl AsRef<[Range]>) -> Self {
        let mut ranges = ranges.as_ref().iter().copied().collect::<Vec<_>>();
        ranges.sort();
        let mut combined = Vec::new();
        let mut current: Option<Range> = None;
        for range in ranges {
            if let Some(cur) = current {
                if let Some(c) = cur.combine(range) {
                    current = Some(c);
                } else {
                    combined.push(cur);
                    current = Some(range);
                }
            } else {
                current = Some(range)
            }
        }

        if let Some(current) = current {
            combined.push(current);
        }

        Self {
            ranges: combined.into_boxed_slice(),
        }
    }

    pub fn contains(&self, value: u64) -> bool {
        let mut low = 0;
        let mut high = self.ranges.len();

        while low < high {
            let mid = (low + high) / 2;
            let range = &self.ranges[mid];
            if range.contains(value) {
                return true;
            } else if value < range.from {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let lookup = Lookup::from([Range::new(1, 4), Range::new(6, 8)]);
        assert!(lookup.contains(1));
        assert!(lookup.contains(2));
        assert!(lookup.contains(4));
        assert!(!lookup.contains(5));
        assert!(lookup.contains(6));
        assert!(lookup.contains(8));
        assert!(!lookup.contains(9));

        let lookup = Lookup::from([Range::new(1, 4), Range::new(3, 5), Range::new(6, 8)]);
        assert!(lookup.contains(1));
        assert!(lookup.contains(2));
        assert!(lookup.contains(4));
        assert!(lookup.contains(5));
        assert!(lookup.contains(6));
        assert!(lookup.contains(8));
        assert!(!lookup.contains(9));

        let lookup = Lookup::from([
            Range::new(2, 3),
            Range::new(5, 5),
            Range::new(7, 9),
            Range::new(11, 12),
            Range::new(17, 17),
            Range::new(20, 24),
            Range::new(29, 30),
        ]);
        assert!(!lookup.contains(1));
        assert!(lookup.contains(2));
        assert!(!lookup.contains(4));
        assert!(lookup.contains(5));
        assert!(!lookup.contains(6));
        assert!(lookup.contains(8));
        assert!(lookup.contains(17));
        assert!(!lookup.contains(18));
        assert!(lookup.contains(30));
        assert!(!lookup.contains(50));

        let lookup = Lookup::from([]);
        assert!(!lookup.contains(20));
    }
}
