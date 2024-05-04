#![allow(dead_code)]

pub fn largest_power_of_two_that_divides_k(k: i32) -> i32 {
    k & -k
}
