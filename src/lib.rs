//! Integer Sequences
//!
//! This is an implementation of integer sequences in rust
//!
//! [`The Online Encyclopedia of Integer Sequences`]: https://oeis.org/

/// return Zero
///
/// see [https://oeis.org/A000004](https://oeis.org/A000004)
pub fn a000004(index: usize) -> usize {
    return 0 * index;
}

/// return The number of divisors of index
/// This is known as tau(n)
///
/// see [https://oeis.org/A000005](https://oeis.org/A000005)
pub fn a000005(index: usize) -> usize {
    let mut i:usize = 2;
    let mut num:usize = 1;
    let mut n: usize = index;

    while i * i <= index {
        if n % i == 0 {
            let mut e: usize = 0;
            while index % i == 0 {
                e += 1;
                n = n / i;
            }
            num *= e + 1;
        }
        i += 1;
    }

    if n > 1 {
        return num + num;
    } else {
        return num;
    }
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

return index; // for now

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

    #[test]
    fn div_test() {
        assert_eq!(a000005(36),9);
        assert_eq!(a000005(4),3);
        assert_eq!(a000005(7),2);
    }
}
