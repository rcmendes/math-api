use std::collections::HashMap;

pub fn fibonacci(n: u8) -> u128 {
    let mut memory: HashMap<u8, u128> = HashMap::new();

    fibonacci_memoized(n, &mut memory)
}

fn fibonacci_memoized(n: u8, memory: &mut HashMap<u8, u128>) -> u128 {
    if n < 2 {
        return n as u128;
    }

    let memoized_value = memory.get(&n);

    let value = match memoized_value {
        Some(v) => *v,
        None => {
            let value = fibonacci_memoized(n - 1, memory) + fibonacci_memoized(n - 2, memory);
            memory.insert(n, value);
            value
        }
    };

    value
}


#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn should_return_0() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn should_return_1() {
        let mut v = fibonacci(1);
        assert_eq!(v, 1);

        v = fibonacci(2);
        assert_eq!(v, 1);
    }

    #[test]
    fn should_return_5() {
        assert_eq!(fibonacci(5), 5);
    }

    #[test]
    fn should_return_21() {
        assert_eq!(fibonacci(8), 21);
    }
}
