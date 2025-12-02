pub fn is_silly_number_part_two(num: usize) -> bool {
    if num == 0 {
        return true;
    }
    let total_digits = num.ilog10() + 1;
    for digits in 1..total_digits {
        if total_digits % digits > 0 {
            continue;
        }
        let factor = 10_usize.pow(digits);
        let part = num % factor;
        let mut new_num = num;
        loop {
            if new_num == 0 {
                return true;
            }
            if new_num % factor != part {
                break;
            }
            new_num = (new_num - part) / factor;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for n in [0, 12341234, 123123123, 1212121212, 1111111, 100] {
            println!("{} {:?}", n, is_silly_number_part_two(n));
        }

        assert!(is_silly_number_part_two(0));
        assert!(is_silly_number_part_two(12341234));
        assert!(is_silly_number_part_two(123123123));
        assert!(is_silly_number_part_two(1212121212));
        assert!(is_silly_number_part_two(1111111));

        assert!(!is_silly_number_part_two(100));
    }
}
