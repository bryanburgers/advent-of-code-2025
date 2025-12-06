fn main() {
    let input = include_str!("input.txt");
    let value = part_one(input);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("example.txt");
        let value = part_one(input);
        assert_eq!(value, 4277556);
    }
}
