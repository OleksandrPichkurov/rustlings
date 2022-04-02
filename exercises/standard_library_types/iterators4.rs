// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // let a = vec![1..=num].iter();
    // a.next().map(|i| i + {i.next()}) as u64 # dont work
    // (1..=num).sum1() # also dont work
    // (1..=num).fold(1, |prev, next| prev * next) # work from standart lib
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

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
