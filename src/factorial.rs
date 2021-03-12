use std::collections::HashMap;

pub fn factorial(n: u8) -> u128 {
    let mut memory: HashMap<u8, u128> = HashMap::new();

    factorial_memoized(n, &mut memory)
}

fn factorial_memoized(n: u8, memory: &mut HashMap<u8, u128>) -> u128 {
    if n < 2 {
        return 1;
    }

    let memoized_value = memory.get(&n);

    let value = match memoized_value {
        Some(v) => *v,
        None => {
            let value = (n as u128) * factorial_memoized(n - 1, memory);
            memory.insert(n, value);
            value
        }
    };

    value
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn should_return_1() {
        let mut v = factorial(0);
        assert_eq!(v, 1);

        v = factorial(1);
        assert_eq!(v, 1);
    }

    #[test]
    fn should_return_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn should_return_120() {
        assert_eq!(factorial(5), 120);
    }
}
