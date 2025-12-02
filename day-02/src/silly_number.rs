/// A number that repeats itself
///
/// This struct can *only* represent silly numbers; it is impossible to construct one of these in
/// such a way that it isn't a silly number.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SillyNumber(usize);

impl SillyNumber {
    pub const ZERO: SillyNumber = SillyNumber(0);

    fn from_base(base: usize) -> Self {
        SillyNumber(base)
    }

    /// Get the value of the number
    pub fn value(self) -> usize {
        let val = self.0;
        if val == 0 {
            return 0;
        }

        let number_of_digits = val.ilog10() + 1;
        let factor_of_ten = 10_usize.pow(number_of_digits);
        let duplicated = factor_of_ten * val;

        duplicated + val
    }

    /// Get the silly number that follows this silly number
    pub fn successor(self) -> Self {
        Self(self.0 + 1)
    }

    /// Get the next silly number that has a value equal to or greater than the provided value
    pub fn next(val: usize) -> Self {
        if val == 0 {
            return Self(0);
        }

        let digits = val.ilog10() + 1;
        let half_digits = digits / 2;
        if digits % 2 == 1 {
            let base = 10_usize.pow(half_digits);
            return Self::from_base(base);
        }

        let base = 10_usize.pow(half_digits);
        let something = val / base;
        let a = SillyNumber::from_base(something);
        if val <= a.value() { a } else { a.successor() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silly_number_from_base_value() {
        assert_eq!(SillyNumber::from_base(0).value(), 0);
        assert_eq!(SillyNumber::from_base(1).value(), 11);
        assert_eq!(SillyNumber::from_base(2).value(), 22);
        assert_eq!(SillyNumber::from_base(9).value(), 99);
        assert_eq!(SillyNumber::from_base(10).value(), 1010);
        assert_eq!(SillyNumber::from_base(11).value(), 1111);
        assert_eq!(SillyNumber::from_base(99).value(), 9999);
        assert_eq!(SillyNumber::from_base(100).value(), 100100);
        assert_eq!(SillyNumber::from_base(1000).value(), 10001000);
        assert_eq!(SillyNumber::from_base(58349105).value(), 5834910558349105);
    }

    #[test]
    fn silly_number_succ() {
        let mut a = SillyNumber::from_base(0);
        assert_eq!(a.value(), 0);
        a = a.successor();
        assert_eq!(a.value(), 11);
        a = a.successor();
        assert_eq!(a.value(), 22);
        a = a.successor();
        assert_eq!(a.value(), 33);
    }

    #[test]
    fn silly_number_next() {
        assert_eq!(SillyNumber::next(0).value(), 0);
        assert_eq!(SillyNumber::next(1).value(), 11);
        assert_eq!(SillyNumber::next(2).value(), 11);
        assert_eq!(SillyNumber::next(10).value(), 11);
        assert_eq!(SillyNumber::next(11).value(), 11);
        assert_eq!(SillyNumber::next(12).value(), 22);
        assert_eq!(SillyNumber::next(98).value(), 99);
        assert_eq!(SillyNumber::next(99).value(), 99);
        assert_eq!(SillyNumber::next(100).value(), 1010);
        assert_eq!(SillyNumber::next(999).value(), 1010);
        assert_eq!(SillyNumber::next(1000).value(), 1010);
        assert_eq!(SillyNumber::next(1010).value(), 1010);
        assert_eq!(SillyNumber::next(1011).value(), 1111);
        assert_eq!(SillyNumber::next(5000).value(), 5050);
        assert_eq!(SillyNumber::next(9998).value(), 9999);
        assert_eq!(SillyNumber::next(9999).value(), 9999);
        assert_eq!(SillyNumber::next(50000).value(), 100100);
        assert_eq!(SillyNumber::next(500000).value(), 500500);
        assert_eq!(SillyNumber::next(500501).value(), 501501);
        assert_eq!(SillyNumber::next(35139581).value(), 35143514);
        assert_eq!(SillyNumber::next(351395814).value(), 1000010000);
    }
}
