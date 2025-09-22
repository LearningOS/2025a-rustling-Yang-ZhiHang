// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // let mut sum = 1;
    // let mut tmp = num;
    // while tmp != 0 {
    //     sum *= tmp;
    //     tmp -= 1;
    // }
    // sum
    
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // if (num == 1 || num == 0) {
    //     1
    // } else {
    //     factorial(num - 1) * num
    // }

    // For an extra challenge, don't use:
    // - recursion
    (1..=num).product()
    // Execute `rustlings hint iterators4` for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
