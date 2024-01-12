#[cfg(test)]
mod tests;

// https://www.geeksforgeeks.org/batch/dsa-4/track/DSASP-BitMagic/video/Nzc0NA%3D%3D
pub fn count_set_bits(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

// https://www.geeksforgeeks.org/batch/dsa-4/track/DSASP-BitMagic/video/MTQ5MDU%3D
pub fn get_2_odd_repeats(v: Vec<i32>) -> [i32; 2] {
    let xor_all = v.iter().fold(0, |acc, e| acc ^ e);
    let last_set_bit = xor_all & -xor_all;

    let mut result = [0; 2];
    for e in v {
        if e & last_set_bit == 0 {
            result[0] ^= e;
        } else {
            result[1] ^= e;
        }
    }
    result
}

// https://www.geeksforgeeks.org/batch/dsa-4/track/DSASP-BitMagic/problem/count-total-set-bits-1587115620
/// Given a number N. Find the total count of set bits for all numbers from 1 to N(both inclusive).
pub fn count_total_setbits_till_n(n: usize) -> usize {
    let mut power: usize = 2;
    let mut count: usize = 0;

    let mut n_copy = n;
    while n_copy > 0 {
        count += ((n + 1) / power) * power / 2;
        count += ((n + 1) % power).max(power / 2) - power / 2;

        power *= 2;
        n_copy >>= 1;
    }

    count
}

pub fn test_power_of_2(n: u128) -> bool {
    return n & (n - 1) == 0;
}

pub fn sparse_bits(n: usize) -> bool {
    return (n & (n >> 1)) == 0;
}
