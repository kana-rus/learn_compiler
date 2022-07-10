use std::str::Chars;

pub fn strtol(chars: &mut Chars) -> (Option<u32>, Option<char>) {
    let mut digits = Vec::<u32>::new();
    loop {
        if let Some(next_char) = (*chars).next() {
            if let Some(digit) = next_char.to_digit(10) {
                digits.push(digit);
            } else {
                let ret = if digits.is_empty() {
                    (None, Some(next_char))
                } else {
                    (
                        Some(digits.iter().fold(0, |a, b| 10*a + b)),
                        Some(next_char)
                    )
                };
                digits.clear();
                return ret;
            }
        } else {
            if digits.is_empty() {
                return (None, None);
            } else {
                return (
                    Some(digits.iter().fold(0, |a, b| 10*a + b)),
                    None
                );
            }
        }
    }
}

pub fn report_unexpected_token(token: char) {
    println!("unecpected token: {}", token);
    panic!();
}