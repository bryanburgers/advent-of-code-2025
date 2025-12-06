use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");
    let value = part_one(input);
    println!("{value}");
    let value = part_two(input);
    println!("{value}");
}

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let lines = input.trim().lines();
    let mut stacks: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        for (idx, val) in line.split_ascii_whitespace().enumerate() {
            if idx >= stacks.len() {
                stacks.push(Vec::new());
            }
            let stack = stacks.get_mut(idx).unwrap();
            if val == "*" {
                sum += stack.iter().fold(1, |a, b| a * *b);
            } else if val == "+" {
                sum += stack.iter().fold(0, |a, b| a + *b);
            } else if let Ok(num) = val.parse() {
                stack.push(num)
            }
        }
    }

    sum
}

fn part_two(input: &str) -> usize {
    let mut sum = 0;

    let roll = Roll::from(input);

    for problem in roll.problem_ranges() {
        let mut numbers = Vec::new();
        let start = problem.start;
        for column in problem {
            numbers.push(roll.number_at_column(column));
        }
        let solution = match roll.operations.as_bytes()[start] {
            b'+' => numbers.iter().fold(0, |a, b| a + b),
            b'*' => numbers.iter().fold(1, |a, b| a * b),
            _ => panic!("Unexpected operation"),
        };

        sum += solution
    }

    sum
}

struct Roll<'a> {
    rows: Vec<&'a str>,
    operations: &'a str,
}

impl<'a> From<&'a str> for Roll<'a> {
    fn from(value: &'a str) -> Self {
        let mut numbers = Vec::new();
        for line in value.lines() {
            if line.starts_with("*") || line.starts_with("+") {
                return Roll {
                    rows: numbers,
                    operations: line,
                };
            } else {
                numbers.push(line);
            }
        }

        panic!("Unexpected");
    }
}

impl Roll<'_> {
    pub fn number_at_column(&self, column: usize) -> usize {
        let mut num = 0;
        for row in &self.rows {
            match row.as_bytes().get(column) {
                None => {
                    panic!("Out of range");
                }
                Some(b' ') if num > 0 => {
                    break;
                }
                Some(b' ') => {}
                Some(v @ b'0'..=b'9') => {
                    let digit = (v - b'0') as usize;
                    num = num * 10 + digit;
                }
                Some(c) => {
                    panic!("Unexpected char {c:x}");
                }
            }
        }
        num
    }

    pub fn problem_ranges(&self) -> ProblemRangesIter<'_> {
        ProblemRangesIter::new(self.operations)
    }
}

pub struct ProblemRangesIter<'a> {
    str: &'a [u8],
    next_start: usize,
}

impl<'a> ProblemRangesIter<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str: str.as_bytes(),
            next_start: 0,
        }
    }
}

impl<'a> Iterator for ProblemRangesIter<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_start >= self.str.len() {
            return None;
        }

        let start = self.next_start;
        loop {
            self.next_start += 1;
            if self.next_start >= self.str.len() {
                self.next_start = self.next_start + 1;
                break;
            }

            match self.str[self.next_start] {
                b' ' => {}
                b'+' | b'*' => {
                    break;
                }
                c => {
                    panic!("Unexpected char {c:x} in problem ranges iter");
                }
            }
        }

        Some(start..self.next_start - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        let value = part_one(input);
        assert_eq!(value, 4277556);
    }

    #[test]
    fn test_roll_number_at_column() {
        let input = r#"
1     1 124  
24   23  35  
356 456   6 4
+   +   +    "#;
        // Remove leading newline
        let input = &input[1..];
        assert!(input.starts_with("1"));

        let roll = Roll::from(input);
        assert_eq!(roll.number_at_column(0), 123);
        assert_eq!(roll.number_at_column(1), 45);
        assert_eq!(roll.number_at_column(2), 6);

        assert_eq!(roll.number_at_column(4), 4);
        assert_eq!(roll.number_at_column(5), 25);
        assert_eq!(roll.number_at_column(6), 136);

        assert_eq!(roll.number_at_column(8), 1);
        assert_eq!(roll.number_at_column(9), 23);
        assert_eq!(roll.number_at_column(10), 456);

        assert_eq!(roll.number_at_column(12), 4);
    }

    #[test]
    fn test_roll_problem_ranges() {
        let input = r#"
1     1
24   23
356 456
+   +  "#;
        // Remove leading newline
        let input = &input[1..];
        assert!(input.starts_with("1"));

        let roll = Roll::from(input);
        let mut ranges = roll.problem_ranges();
        assert_eq!(ranges.next(), Some(0..3));
        assert_eq!(ranges.next(), Some(4..7));
        assert_eq!(ranges.next(), None);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("example.txt");
        let value = part_two(input);
        assert_eq!(value, 3263827);
    }
}
