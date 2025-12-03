fn main() {
    let input = include_str!("input.txt");
    let value = solve(input, 2);
    println!("{value}");
    let value = solve(input, 12);
    println!("{value}");
}

fn solve(s: &str, count: usize) -> usize {
    let lines = s.trim().lines();
    let mut sum = 0;
    for line in lines {
        let v = max_joltage(line.trim(), count);
        sum += v;
    }
    sum
}

fn max_joltage(s: &str, count: usize) -> usize {
    fn max_rec(s: &str, count: usize, accum: usize) -> usize {
        if count == 0 {
            return accum;
        }

        let bytes = s.as_bytes();
        let len = s.len();
        let mut max_value = 0;
        let mut max_index = 0;
        for i in 0..=(len - count) {
            let byte = bytes[i];
            if byte > max_value {
                max_value = byte;
                max_index = i;
            }
        }
        let val = (max_value - b'0') as usize;
        let shifted_val = 10_usize.pow((count - 1) as u32) * val;

        max_rec(&s[max_index + 1..], count - 1, shifted_val + accum)
    }

    max_rec(s, count, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage("987654321111111", 2), 98);
        assert_eq!(max_joltage("811111111111119", 2), 89);
        assert_eq!(max_joltage("234234234234278", 2), 78);
        assert_eq!(max_joltage("818181911112111", 2), 92);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        assert_eq!(solve(input, 2), 357);
    }

    #[test]
    fn test_max_joltage_twelve() {
        assert_eq!(max_joltage("987654321111111", 12), 987654321111);
        assert_eq!(max_joltage("811111111111119", 12), 811111111119);
        assert_eq!(max_joltage("234234234234278", 12), 434234234278);
        assert_eq!(max_joltage("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("example.txt");
        assert_eq!(solve(input, 12), 3121910778619);
    }
}
