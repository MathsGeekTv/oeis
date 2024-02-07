//! Integer Sequences
//!
//! This is an implementation of integer sequences in rust
//!
//! [`The Online Encyclopedia of Integer Sequences`]: https://oeis.org/

/// return Zero
///
/// see [https://oeis.org/A000004](https://oeis.org/A000004)
pub fn a000004(index: usize) -> usize {
    return 0;
}


/// return the fibonacci at position index
///
/// see [https://oeis.org/A000045](https://oeis.org/A000045)
pub fn a000045(index: usize) -> usize {
    match index {
        0 =>  0,
        1 =>  1,
        2 =>  1,
        _ =>  a000045(index - 2 ) + a000045( index - 1),
    }
}

///  return the fibonacci at position index using the closed form rather than recursion
///
/// see [a000045]
/// Wikipedia [Closed Form Value](https://en.wikipedia.org/wiki/Fibonacci_sequence#Relation_to_the_golden_ratio)

pub fn a000045_cf(index: usize) -> usize {


}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn fib_test() {
        assert_eq!(a000045(1), 1);
        assert_eq!(a000045(12), 144);
    }
}
