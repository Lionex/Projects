use std::env;

fn main() {
    collatz(90000000000);
}

fn collatz(n: i64) -> i64 {
    assert!(n >= 1, "Must be a positive number");
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2)   }
        _ => { 1 + collatz(n*3+1) }
    }
}

#[cfg(test)]
mod collatz_test {
    use super::*;

    #[test]
    fn collatz_of_1() {
        assert_eq!(collatz(1), 0, "collatz(1) returned nonzero value {}", collatz(1));
    }

    #[test]
    fn known_length() {
        fn compare_result(val: i64, expect: i64) {
            assert_eq!( collatz(val), expect
                , "expect collatz({}) == {}, not {}", val, expect, collatz(val)
            );
        }

        compare_result(10, 6);
        compare_result(361, 45);
        compare_result(18514, 260);
        compare_result(63728127, 949);
        compare_result(9780657630, 1132);
    }

    #[test]
    #[should_panic]
    fn catch_undefined() {
        collatz(0);
        collatz(-132);
    }
}
